import * as chrono from 'chrono-node';

// Day shorthands mapping (case-insensitive)
const DAY_SHORTHANDS: Record<string, string> = {
  tod: 'today',
  tom: 'tomorrow',
  yes: 'yesterday',
  mon: 'monday',
  tue: 'tuesday',
  wed: 'wednesday',
  thu: 'thursday',
  fri: 'friday',
  sat: 'saturday',
  sun: 'sunday',
};

// Build regex pattern that matches shorthands as whole words
const shorthandPattern = new RegExp(
  `\\b(${Object.keys(DAY_SHORTHANDS).join('|')})\\b`,
  'gi'
);

export interface ParsedDate {
  text: string;
  start: number;
  end: number;
  date: Date;
  dateString: string;
}

export interface ParseResult {
  cleanedText: string;
  parsedDates: ParsedDate[];
  primaryDate: ParsedDate | null;
}

interface ShorthandReplacement {
  originalStart: number;
  originalEnd: number;
  originalText: string;
  expandedStart: number;
  expandedEnd: number;
  expandedText: string;
}

function expandShorthandsSimple(text: string): { expanded: string; replacements: ShorthandReplacement[] } {
  const replacements: ShorthandReplacement[] = [];
  let expanded = '';
  let lastIndex = 0;
  let cumOffset = 0;

  shorthandPattern.lastIndex = 0;
  let match;

  while ((match = shorthandPattern.exec(text)) !== null) {
    const shorthand = match[0].toLowerCase();
    const replacement = DAY_SHORTHANDS[shorthand];

    expanded += text.slice(lastIndex, match.index);
    const expandedStart = match.index + cumOffset;

    replacements.push({
      originalStart: match.index,
      originalEnd: match.index + match[0].length,
      originalText: match[0],
      expandedStart,
      expandedEnd: expandedStart + replacement.length,
      expandedText: replacement,
    });

    expanded += replacement;
    cumOffset += replacement.length - match[0].length;
    lastIndex = match.index + match[0].length;
  }

  expanded += text.slice(lastIndex);
  return { expanded, replacements };
}

function mapExpandedToOriginal(
  expandedStart: number,
  expandedEnd: number,
  replacements: ShorthandReplacement[]
): { start: number; end: number } {
  let startOffset = 0;
  let endOffset = 0;

  for (const r of replacements) {
    // For start position
    if (expandedStart > r.expandedEnd) {
      startOffset += r.originalText.length - r.expandedText.length;
    } else if (expandedStart >= r.expandedStart && expandedStart < r.expandedEnd) {
      // Start is inside a replacement - map to the original shorthand start
      startOffset += r.originalStart - r.expandedStart;
    }

    // For end position
    if (expandedEnd > r.expandedEnd) {
      endOffset += r.originalText.length - r.expandedText.length;
    } else if (expandedEnd > r.expandedStart && expandedEnd <= r.expandedEnd) {
      // End is inside a replacement - map to the original shorthand end
      endOffset += r.originalEnd - r.expandedEnd;
    }
  }

  return {
    start: expandedStart + startOffset,
    end: expandedEnd + endOffset,
  };
}

export function parseNaturalDate(text: string): ParseResult {
  // Expand shorthands for chrono parsing
  const { expanded, replacements } = expandShorthandsSimple(text);

  const results = chrono.parse(expanded, new Date(), { forwardDate: true });

  const parsedDates: ParsedDate[] = results.map((result) => {
    const date = result.start.date();
    const expandedStart = result.index;
    const expandedEnd = result.index + result.text.length;

    // Map positions back to original text
    const { start, end } = mapExpandedToOriginal(expandedStart, expandedEnd, replacements);

    return {
      text: text.slice(start, end), // Use original text (with shorthand)
      start,
      end,
      date,
      dateString: formatDate(date),
    };
  });

  // Remove date text from the original string (for task content)
  let cleanedText = text;
  // Process from end to start to preserve indices
  for (let i = parsedDates.length - 1; i >= 0; i--) {
    const parsed = parsedDates[i];
    const before = cleanedText.slice(0, parsed.start);
    const after = cleanedText.slice(parsed.end);
    cleanedText = before + after;
  }

  // Clean up extra whitespace
  cleanedText = cleanedText.replace(/\s+/g, ' ').trim();

  return {
    cleanedText,
    parsedDates,
    primaryDate: parsedDates.length > 0 ? parsedDates[0] : null,
  };
}

export function formatDate(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

export function getRelativeDateLabel(date: Date): string {
  const today = new Date();
  today.setHours(0, 0, 0, 0);

  const target = new Date(date);
  target.setHours(0, 0, 0, 0);

  const diffTime = target.getTime() - today.getTime();
  const diffDays = Math.round(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Tomorrow';
  if (diffDays === -1) return 'Yesterday';
  if (diffDays > 1 && diffDays <= 7) return `In ${diffDays} days`;
  if (diffDays < -1 && diffDays >= -7) return `${Math.abs(diffDays)} days ago`;

  return formatDate(date);
}

export interface HighlightSegment {
  text: string;
  isDate: boolean;
  dateInfo?: ParsedDate;
}

export function getHighlightedSegments(text: string): HighlightSegment[] {
  const result = parseNaturalDate(text);
  const segments: HighlightSegment[] = [];

  if (result.parsedDates.length === 0) {
    return [{ text, isDate: false }];
  }

  // Sort by start position
  const sortedDates = [...result.parsedDates].sort((a, b) => a.start - b.start);

  let lastEnd = 0;
  for (const parsed of sortedDates) {
    // Add non-date text before this date
    if (parsed.start > lastEnd) {
      segments.push({
        text: text.slice(lastEnd, parsed.start),
        isDate: false,
      });
    }

    // Add the date segment
    segments.push({
      text: parsed.text,
      isDate: true,
      dateInfo: parsed,
    });

    lastEnd = parsed.end;
  }

  // Add remaining text after last date
  if (lastEnd < text.length) {
    segments.push({
      text: text.slice(lastEnd),
      isDate: false,
    });
  }

  return segments;
}
