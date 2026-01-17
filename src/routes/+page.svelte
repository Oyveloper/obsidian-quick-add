<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import QuickInput from '$lib/components/QuickInput.svelte';

  interface Vault {
    id: string;
    path: string;
    name: string;
  }

  let vaults: Vault[] = $state([]);
  let selectedVault: Vault | null = $state(null);
  let error: string | null = $state(null);
  let success: string | null = $state(null);
  let showVaultSelector = $state(false);

  const VAULT_STORAGE_KEY = 'obsidian-quick-add-vault';

  onMount(async () => {
    try {
      vaults = await invoke<Vault[]>('get_vaults');
      if (vaults.length > 0) {
        // Try to restore saved vault selection
        const savedVaultId = localStorage.getItem(VAULT_STORAGE_KEY);
        if (savedVaultId) {
          const savedVault = vaults.find(v => v.id === savedVaultId);
          if (savedVault) {
            selectedVault = savedVault;
            return;
          }
        }
        selectedVault = vaults[0];
      }
    } catch (e) {
      error = String(e);
    }
  });

  function selectVault(vault: Vault) {
    selectedVault = vault;
    localStorage.setItem(VAULT_STORAGE_KEY, vault.id);
    showVaultSelector = false;
  }

  async function handleSubmit(
    originalText: string,
    cleanedText: string,
    dateString: string | null
  ) {
    if (!selectedVault) {
      error = 'No vault selected';
      return;
    }

    try {
      error = null;
      success = null;

      await invoke('add_task_to_daily_note', {
        vaultPath: selectedVault.path,
        taskContent: cleanedText,
        dueDate: dateString,
      });

      success = 'Task added!';

      // Hide window after success
      setTimeout(async () => {
        success = null;
        await invoke('hide_window');
      }, 500);
    } catch (e) {
      error = String(e);
    }
  }

  async function handleCancel() {
    if (showVaultSelector) {
      showVaultSelector = false;
      return;
    }
    error = null;
    success = null;
    await invoke('hide_window');
  }
</script>

<main class="app-container">
  {#if showVaultSelector}
    <div class="vault-selector">
      <span class="vault-label">Select vault:</span>
      {#each vaults as vault}
        <button
          class="vault-option"
          class:selected={selectedVault?.id === vault.id}
          onclick={() => selectVault(vault)}
        >
          {vault.name}
        </button>
      {/each}
    </div>
  {:else if error}
    <div class="error-message">{error}</div>
  {:else if success}
    <div class="success-message">{success}</div>
  {:else}
    <QuickInput onSubmit={handleSubmit} onCancel={handleCancel} />
    <button class="vault-indicator" onclick={() => showVaultSelector = true} title="Click to change vault">
      {selectedVault?.name ?? 'No vault'}
    </button>
  {/if}
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(html, body) {
    height: 100%;
    background: transparent;
    overflow: hidden;
  }

  .app-container {
    width: 100%;
    height: 64px;
    background: #ffffff;
    border-radius: 2px;
    border: 2px solid #1a1a1a;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 20px;
    gap: 12px;
    box-shadow: 4px 4px 0 #1a1a1a;
  }

  .vault-indicator {
    flex-shrink: 0;
    padding: 4px 6px;
    background: transparent;
    border: none;
    color: #a3a3a3;
    font-size: 11px;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    cursor: pointer;
    transition: color 0.15s ease;
    max-width: 80px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .vault-indicator:hover {
    color: #525252;
  }

  .vault-selector {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 8px;
    overflow-x: auto;
    width: 100%;
  }

  .vault-label {
    color: #525252;
    font-size: 13px;
    font-weight: 600;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    white-space: nowrap;
  }

  .vault-option {
    padding: 6px 12px;
    background: #ffffff;
    border: 2px solid #1a1a1a;
    border-radius: 0;
    color: #1a1a1a;
    font-size: 13px;
    font-weight: 500;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    cursor: pointer;
    transition: all 0.1s ease;
    white-space: nowrap;
  }

  .vault-option:hover {
    background: #f5f5f5;
  }

  .vault-option.selected {
    background: #1a1a1a;
    color: #ffffff;
  }

  .error-message {
    color: #dc2626;
    font-size: 13px;
    font-weight: 500;
    padding: 0 16px;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  }

  .success-message {
    color: #16a34a;
    font-size: 14px;
    font-weight: 600;
    padding: 0 16px;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  }
</style>
