console.log('ClipSync main.js loading...');
console.log('Tauri available:', !!window.__TAURI__);

const { invoke } = window.__TAURI__.core;

let isConnected = false;
let isPaused = false;

console.log('invoke function:', typeof invoke);

// Wait for DOM to be ready
document.addEventListener('DOMContentLoaded', () => {
    console.log('DOM loaded, initializing...');
    initializeApp();
});

function initializeApp() {
    console.log('Initializing app...');
    
    // DOM elements
    const statusIndicator = document.getElementById('statusIndicator');
    const statusText = document.getElementById('statusText');
    const serverUrlInput = document.getElementById('serverUrl');
    const sharedSecretInput = document.getElementById('sharedSecret');
    const connectBtn = document.getElementById('connectBtn');
    const disconnectBtn = document.getElementById('disconnectBtn');
    const pauseBtn = document.getElementById('pauseBtn');
    const historyList = document.getElementById('historyList');
    const messageArea = document.getElementById('messageArea');
    const autostartCheck = document.getElementById('autostartCheck');
    const startMinimizedCheck = document.getElementById('startMinimizedCheck');
    const hideBtn = document.getElementById('hideBtn');
    
    console.log('Connect button found:', !!connectBtn);

// Load saved configuration
async function loadConfig() {
    try {
        const config = await invoke('get_config');
        if (config.server_url) {
            serverUrlInput.value = config.server_url;
        }
        if (config.shared_secret) {
            sharedSecretInput.value = config.shared_secret;
        }
        autostartCheck.checked = config.autostart || false;
        startMinimizedCheck.checked = config.start_minimized || false;
        return config;
    } catch (e) {
        console.error('Failed to load config:', e);
        return null;
    }
}

// Show message
function showMessage(text, type = 'success') {
    const msg = document.createElement('div');
    msg.className = `message ${type}`;
    msg.textContent = text;
    messageArea.innerHTML = '';
    messageArea.appendChild(msg);
    
    setTimeout(() => {
        msg.remove();
    }, 3000);
}

// Update status
function updateStatus(connected) {
    isConnected = connected;
    
    if (connected) {
        statusIndicator.classList.add('connected');
        statusText.textContent = 'Connected';
        connectBtn.disabled = true;
        disconnectBtn.disabled = false;
        pauseBtn.disabled = false;
        serverUrlInput.disabled = true;
        sharedSecretInput.disabled = true;
    } else {
        statusIndicator.classList.remove('connected');
        statusText.textContent = 'Disconnected';
        connectBtn.disabled = false;
        disconnectBtn.disabled = true;
        pauseBtn.disabled = true;
        serverUrlInput.disabled = false;
        sharedSecretInput.disabled = false;
    }
}

// Add history item to UI
function addHistoryItem(item, timestamp) {
    if (historyList.querySelector('p')) {
        historyList.innerHTML = '';
    }
    
    const div = document.createElement('div');
    div.className = 'history-item';
    
    if (item.type === 'Text') {
        const preview = item.data.length > 100 ? item.data.substring(0, 100) + '...' : item.data;
        div.textContent = preview;
    } else if (item.type === 'Image') {
        div.className += ' image';
        div.textContent = `[Image: ${item.data.mime_type}]`;
    }
    
    const time = new Date(timestamp);
    const timeDiv = document.createElement('div');
    timeDiv.className = 'timestamp';
    timeDiv.textContent = time.toLocaleTimeString();
    div.appendChild(timeDiv);
    
    historyList.insertBefore(div, historyList.firstChild);
    
    // Keep only last 20 items in UI
    while (historyList.children.length > 20) {
        historyList.removeChild(historyList.lastChild);
    }
}

    // Connect to server
    console.log('Setting up connect button handler, button:', connectBtn);
    connectBtn.addEventListener('click', async () => {
        console.log('Connect button clicked');
        alert('Button clicked!'); // Debug
        const serverUrl = serverUrlInput.value.trim();
        const sharedSecret = sharedSecretInput.value.trim();
        
        console.log('Server URL:', serverUrl);
        console.log('Has secret:', !!sharedSecret);
        
        if (!serverUrl || !sharedSecret) {
            showMessage('Please enter both server URL and shared secret', 'error');
            return;
        }
        
        try {
            console.log('Calling connect_to_server...');
            await invoke('connect_to_server', { serverUrl, sharedSecret });
            console.log('Connect successful');
            updateStatus(true);
            showMessage('Connected successfully');
        } catch (e) {
            console.error('Connection error:', e);
            showMessage(`Connection failed: ${e}`, 'error');
        }
    });

    // Disconnect from server
    disconnectBtn.addEventListener('click', async () => {
    try {
        await invoke('disconnect_from_server');
        updateStatus(false);
        showMessage('Disconnected');
    } catch (e) {
        showMessage(`Disconnect failed: ${e}`, 'error');
    }
});

    // Toggle pause
    pauseBtn.addEventListener('click', async () => {
    try {
        isPaused = !isPaused;
        await invoke('toggle_sync', { paused: isPaused });
        pauseBtn.textContent = isPaused ? 'Resume Sync' : 'Pause Sync';
        showMessage(isPaused ? 'Sync paused' : 'Sync resumed');
    } catch (e) {
        showMessage(`Toggle failed: ${e}`, 'error');
    }
});

    // Hide window to tray
    hideBtn.addEventListener('click', async () => {
    try {
        await invoke('hide_window');
    } catch (e) {
        showMessage(`Failed to hide: ${e}`, 'error');
    }
});

    // Autostart toggle
    autostartCheck.addEventListener('change', async () => {
    try {
        await invoke('set_autostart', { enabled: autostartCheck.checked });
        showMessage(autostartCheck.checked ? 'Autostart enabled' : 'Autostart disabled');
    } catch (e) {
        showMessage(`Failed to set autostart: ${e}`, 'error');
        autostartCheck.checked = !autostartCheck.checked;
    }
});

    // Start minimized toggle
    startMinimizedCheck.addEventListener('change', async () => {
    try {
        await invoke('set_start_minimized', { enabled: startMinimizedCheck.checked });
        showMessage(startMinimizedCheck.checked ? 'Will start minimized' : 'Will start normally');
    } catch (e) {
        showMessage(`Failed to set start minimized: ${e}`, 'error');
        startMinimizedCheck.checked = !startMinimizedCheck.checked;
    }
});

    // Load config and set up event listeners
    loadConfig().then(async (config) => {
        // Auto-connect if server URL and secret are configured
        if (config && config.server_url && config.shared_secret) {
            console.log('Auto-connecting to server...');
            try {
                await invoke('connect_to_server', { 
                    serverUrl: config.server_url, 
                    sharedSecret: config.shared_secret 
                });
                console.log('Auto-connect successful');
                updateStatus(true);
                showMessage('Connected successfully');
            } catch (e) {
                console.error('Auto-connect failed:', e);
                // Don't show error message on auto-connect failure, just log it
            }
        }
    });
    
    // Listen for status updates
    const { listen } = window.__TAURI__.event;
    
    listen('clipboard-received', (event) => {
        const { item, timestamp } = event.payload;
        addHistoryItem(item, timestamp);
    });
    
    listen('connection-status', (event) => {
        updateStatus(event.payload.connected);
        if (!event.payload.connected && event.payload.error) {
            showMessage(event.payload.error, 'error');
        }
    });
    
    listen('history-loaded', (event) => {
        const history = event.payload.history;
        history.forEach(entry => {
            addHistoryItem(entry.item, entry.timestamp);
        });
    });
    
    console.log('App initialization complete');
}
