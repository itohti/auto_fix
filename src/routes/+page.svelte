<script>
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    let isFileLoaded = false;
    let auto_fixed_content = "";

    async function handleFileUpload() {
        try {
            const filePath = await open({
                multiple: false,
                filters: [
                    { name: 'All Files', extensions: ['docx', 'txt'] },
                ]
            });

            if (filePath) {
                isFileLoaded = true;
                auto_fixed_content = await invoke("auto_fix", {path: filePath});
            }
        } catch (error) {
            console.error('Error opening file dialog:', error);
        }
    }
</script>

<div class="flex flex-col h-screen">
    <div class="flex gap-2 p-2">
        <button 
            class="px-4 border border-sky-500 rounded-sm hover:bg-sky-500 hover:text-white transition-colors"
        >
            Auto-fix
        </button>
        <button
            class="px-4 border border-sky-500 rounded-sm hover:bg-sky-500 hover:text-white transition-colors"
            on:click={handleFileUpload}
        >
            Load File
        </button>
    </div>

    <div class="grid grid-cols-2 gap-1 flex-1 overflow-hidden">
        <div class="h-full border overflow-auto">
            {#if isFileLoaded}
                <pre class="whitespace-pre-wrap p-4 font-mono text-sm break-words">{auto_fixed_content}</pre>
            {:else}
                <div class="h-full flex items-center justify-center">
                    <p>No File Uploaded</p>
                </div>
            {/if}
        </div>

        <div class="border p-4">
            <p>Output</p>
        </div>
    </div>
</div>