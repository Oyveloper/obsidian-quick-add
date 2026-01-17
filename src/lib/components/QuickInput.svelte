<script lang="ts">
  import { parseNaturalDate, getHighlightedSegments, type ParseResult } from '$lib/dateParser';
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  interface Props {
    onSubmit: (text: string, cleanedText: string, dateString: string | null) => void;
    onCancel: () => void;
  }

  let { onSubmit, onCancel }: Props = $props();

  let inputValue = $state('');
  let inputElement: HTMLInputElement | null = $state(null);
  let parseResult: ParseResult | null = $derived(
    inputValue ? parseNaturalDate(inputValue) : null
  );
  let segments = $derived(getHighlightedSegments(inputValue));

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && inputValue.trim()) {
      event.preventDefault();
      const result = parseNaturalDate(inputValue);
      onSubmit(
        inputValue,
        result.cleanedText,
        result.primaryDate?.dateString ?? null
      );
      inputValue = '';
    } else if (event.key === 'Escape') {
      event.preventDefault();
      inputValue = '';
      onCancel();
    } else if (event.key === 'u' && event.ctrlKey) {
      event.preventDefault();
      inputValue = '';
    }
  }

  function focusInput() {
    if (inputElement) {
      inputElement.focus();
    }
  }

  // Focus input when element becomes available
  $effect(() => {
    if (inputElement) {
      // Small delay to ensure window is ready
      setTimeout(focusInput, 50);
    }
  });

  onMount(() => {
    // Listen for window focus events
    const appWindow = getCurrentWindow();
    const unlistenPromise = appWindow.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        setTimeout(focusInput, 10);
      } else {
        // Close when window loses focus (clicking outside)
        inputValue = '';
        onCancel();
      }
    });

    return () => {
      unlistenPromise.then(unlisten => unlisten());
    };
  });
</script>

<div class="quick-input-container">
  <div class="input-wrapper">
    <div class="highlight-layer" aria-hidden="true">
      {#each segments as segment}
        {#if segment.isDate}
          <span class="date-highlight">{segment.text}</span>
        {:else}
          <span>{segment.text}</span>
        {/if}
      {/each}
    </div>
    <input
      bind:this={inputElement}
      bind:value={inputValue}
      onkeydown={handleKeydown}
      type="text"
      class="task-input"
      placeholder="Add task..."
      spellcheck="false"
      autocomplete="off"
    />
  </div>
  {#if parseResult?.primaryDate}
    <div class="date-badge">
      <span class="date-icon">&#9203;</span>
      <span class="date-text">{parseResult.primaryDate.dateString}</span>
    </div>
  {/if}
</div>

<style>
  .quick-input-container {
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
    padding: 0;
    box-sizing: border-box;
    gap: 10px;
  }

  .input-wrapper {
    position: relative;
    flex: 1;
    height: 32px;
  }

  .highlight-layer {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    padding: 4px 0;
    font-size: 15px;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-weight: 500;
    line-height: 24px;
    color: transparent;
    white-space: pre;
    overflow: hidden;
    pointer-events: none;
  }

  .highlight-layer .date-highlight {
    background-color: #fde047;
    padding: 1px 4px;
  }

  .task-input {
    position: relative;
    width: 100%;
    height: 100%;
    padding: 4px 0;
    font-size: 15px;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-weight: 500;
    line-height: 24px;
    color: #1a1a1a;
    background: transparent;
    border: none;
    border-bottom: 2px solid #e5e5e5;
    outline: none;
    caret-color: #1a1a1a;
    box-sizing: border-box;
  }

  .task-input::placeholder {
    color: #a3a3a3;
    font-weight: 400;
  }

  .task-input:focus {
    border-bottom-color: #1a1a1a;
  }

  .date-badge {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    background: #1a1a1a;
    border: none;
    color: #ffffff;
    font-size: 12px;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-weight: 600;
    white-space: nowrap;
  }

  .date-icon {
    font-size: 11px;
  }

  .date-text {
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  }
</style>
