/**
 * ChargeMesh Web Inspector — Main Entry Point
 * Built with Emerge Core and WASM
 */

import {
    signal,
    computed,
    effect,
    createOwner,
    runWithOwner,
} from '@emerge/core';

// WASM imports (with type declarations)
// Note: In production, this would be imported from the built WASM package
// For now, we use a dynamic import approach
// @ts-ignore - WASM module will be available at runtime
import init from '../../crates/chargemesh-wasm/pkg/chargemesh_wasm.js';

// ============================================================================
// Types
// ============================================================================

interface ParsedMessage {
    raw: string;
    timestamp: string;
    direction: 'incoming' | 'outgoing';
    message: string;
}

interface AnalysisResult {
    timeline: ParsedMessage[];
    state: {
        state: string;
        connectors: Record<string, string>;
        transactions: number;
    };
    errors: Array<{
        timestamp: string;
        error_code: string;
        error_description: string;
    }>;
    capabilities: any;
}

interface WasmModule {
    parse_ocpp_message: (raw: string) => any;
    analyze_timeline: (messages: any[]) => any;
    run_diagnostics: (messages: any[]) => any;
    analyze_capabilities: (context: any) => any;
    version: () => string;
}

// ============================================================================
// State
// ============================================================================

const messages = signal<ParsedMessage[]>([]);
const isConnected = signal<boolean>(false);
const isAnalyzing = signal<boolean>(false);
const selectedMessageIndex = signal<number | null>(null);
const activeTab = signal<'timeline' | 'state' | 'capabilities' | 'diagnostics'>('timeline');

// Computed states
const errorCount = computed(() => {
    return messages.value.filter((m: ParsedMessage) => 
        m.message.includes('Error') || m.message.includes('Faulted')
    ).length;
});

const messageCount = computed(() => messages.value.length);

const hasMessages = computed(() => messages.value.length > 0);

// ============================================================================
// DOM Refs
// ============================================================================

function $(id: string): HTMLElement {
    const el = document.getElementById(id);
    if (!el) throw new Error(`Element with id "${id}" not found`);
    return el;
}

const timelineContainer = $('timeline-container');
const stateContainer = $('state-container');
const capabilitiesContainer = $('capabilities-container');
const diagnosticsContainer = $('diagnostics-container');
const fileInput = $('file-input') as HTMLInputElement;
const dropZone = $('drop-zone');
const fileName = $('file-name');
const fileMessages = $('file-messages');
const statMessages = $('stat-messages');
const statErrors = $('stat-errors');
const statState = $('stat-state');
const statCapabilities = $('stat-capabilities');
const versionEl = $('version');
const wasmVersionEl = $('wasm-version');
const btnAnalyze = $('btn-analyze') as HTMLButtonElement;
const btnRunAnalysis = $('btn-run-analysis') as HTMLButtonElement;

// ============================================================================
// WASM Initialization
// ============================================================================

let wasmModule: WasmModule | null = null;
let wasmInitialized = false;

async function initWasm() {
    try {
        // Import the WASM module
        const wasm = await init();
        wasmModule = wasm as WasmModule;
        wasmInitialized = true;
        
        if (wasmVersionEl && wasmModule.version) {
            wasmVersionEl.textContent = wasmModule.version();
        }
        console.log('✅ WASM initialized');
    } catch (e) {
        console.error('❌ Failed to initialize WASM:', e);
        if (wasmVersionEl) {
            wasmVersionEl.textContent = 'failed to load';
        }
        showError('Failed to load WASM module. Please ensure the WASM build exists.');
    }
}

// ============================================================================
// WASM Wrapper Functions
// ============================================================================

function callWasm<T>(fn: (mod: WasmModule) => T, fallback: T): T {
    if (wasmInitialized && wasmModule) {
        try {
            return fn(wasmModule);
        } catch (e) {
            console.error('WASM call failed:', e);
            return fallback;
        }
    }
    return fallback;
}

function parseOcppMessage(raw: string): any {
    return callWasm(
        (mod) => mod.parse_ocpp_message(raw),
        { success: false, error: 'WASM not loaded' }
    );
}

function analyzeTimeline(messages: ParsedMessage[]): any {
    return callWasm(
        (mod) => mod.analyze_timeline(messages),
        { timeline: [], state: { state: 'Unknown', connectors: {}, transactions: 0 }, errors: [], capabilities: {} }
    );
}

function runDiagnostics(messages: ParsedMessage[]): any {
    return callWasm(
        (mod) => mod.run_diagnostics(messages),
        { report: null, has_issues: false }
    );
}

// ============================================================================
// Effects
// ============================================================================

const owner = createOwner();

runWithOwner(owner, () => {
    // Update stats
    effect(() => {
        const count = messageCount.value;
        const errors = errorCount.value;
        statMessages.textContent = String(count);
        statErrors.textContent = String(errors);
        fileMessages.textContent = `${count} messages`;
        btnAnalyze.disabled = count === 0;
        btnRunAnalysis.disabled = count === 0;
    });

    // Update version
    effect(() => {
        versionEl.textContent = `v${'0.1.0'}`;
    });
});

// ============================================================================
// Tab Switching
// ============================================================================

document.querySelectorAll('.tab').forEach((tab) => {
    tab.addEventListener('click', () => {
        const tabId = tab.getAttribute('data-tab') as typeof activeTab.value;
        if (!tabId) return;

        // Update active tab
        document.querySelectorAll('.tab').forEach((t) => t.classList.remove('active'));
        tab.classList.add('active');

        // Update content
        document.querySelectorAll('.tab-content').forEach((el) => {
            (el as HTMLElement).style.display = 'none';
        });
        const content = $(`tab-${tabId}`);
        if (content) content.style.display = 'block';

        activeTab.value = tabId;
    });
});

// ============================================================================
// File Upload
// ============================================================================

dropZone.addEventListener('click', () => fileInput.click());
dropZone.addEventListener('dragover', (e) => {
    e.preventDefault();
    dropZone.style.borderColor = 'var(--color-accent)';
});
dropZone.addEventListener('dragleave', () => {
    dropZone.style.borderColor = 'var(--border-color)';
});
dropZone.addEventListener('drop', (e) => {
    e.preventDefault();
    dropZone.style.borderColor = 'var(--border-color)';
    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
        loadFile(files[0]);
    }
});

fileInput.addEventListener('change', () => {
    if (fileInput.files && fileInput.files.length > 0) {
        loadFile(fileInput.files[0]);
    }
});

async function loadFile(file: File) {
    fileName.textContent = file.name;
    const content = await file.text();

    // Parse messages
    const parsed: ParsedMessage[] = [];
    const lines = content.split('\n');

    for (const line of lines) {
        if (line.trim()) {
            try {
                const result = parseOcppMessage(line);
                if (result && result.success !== false) {
                    parsed.push({
                        raw: line,
                        timestamp: result.timestamp || new Date().toISOString(),
                        direction: result.direction === 'Incoming' ? 'incoming' : 'outgoing',
                        message: result.message || line.substring(0, 50),
                    });
                }
            } catch (e) {
                // Skip invalid messages
            }
        }
    }

    messages.value = parsed;
    renderTimeline();
}

// ============================================================================
// Live Capture
// ============================================================================

let ws: WebSocket | null = null;
const wsUrlInput = $('ws-url') as HTMLInputElement;
const btnConnect = $('btn-connect') as HTMLButtonElement;
const btnDisconnect = $('btn-disconnect') as HTMLButtonElement;
const liveStatusText = $('live-status-text');
const statusDot = document.querySelector('.status-dot');

btnConnect.addEventListener('click', () => {
    if (!wsUrlInput.value) return;
    connectWebSocket(wsUrlInput.value);
});

btnDisconnect.addEventListener('click', () => {
    disconnectWebSocket();
});

function connectWebSocket(url: string) {
    if (ws) return;

    ws = new WebSocket(url);
    setLiveStatus('connecting', 'Connecting...');

    ws.onopen = () => {
        setLiveStatus('connected', 'Connected');
        isConnected.value = true;
        btnConnect.disabled = true;
        btnDisconnect.disabled = false;
    };

    ws.onmessage = (event) => {
        const raw = event.data;
        try {
            const result = parseOcppMessage(raw);
            if (result && result.success !== false) {
                const newMsg: ParsedMessage = {
                    raw,
                    timestamp: result.timestamp || new Date().toISOString(),
                    direction: result.direction === 'Incoming' ? 'incoming' : 'outgoing',
                    message: result.message || raw.substring(0, 50),
                };
                messages.value = [...messages.value, newMsg];
                renderTimeline();
            }
        } catch (e) {
            // Skip
        }
    };

    ws.onerror = () => {
        setLiveStatus('error', 'Error');
        disconnectWebSocket();
    };

    ws.onclose = () => {
        disconnectWebSocket();
    };
}

function disconnectWebSocket() {
    if (ws) {
        ws.close();
        ws = null;
    }
    isConnected.value = false;
    btnConnect.disabled = false;
    btnDisconnect.disabled = true;
    setLiveStatus('disconnected', 'Disconnected');
}

function setLiveStatus(state: string, text: string) {
    if (statusDot) {
        statusDot.className = `status-dot ${state}`;
    }
    if (liveStatusText) {
        liveStatusText.textContent = text;
    }
}

// ============================================================================
// Analysis
// ============================================================================

btnAnalyze.addEventListener('click', runAnalysis);
btnRunAnalysis.addEventListener('click', runAnalysis);

async function runAnalysis() {
    if (!wasmInitialized || messages.value.length === 0) return;

    isAnalyzing.value = true;
    btnRunAnalysis.textContent = '⏳ Analyzing...';
    btnRunAnalysis.disabled = true;

    try {
        // Run full analysis
        const data = analyzeTimeline(messages.value);

        if (data) {
            // Update state
            if (data.state) {
                statState.textContent = data.state.state || '—';
                renderStateMachine(data.state);
            }

            // Update capabilities
            if (data.capabilities) {
                const caps = data.capabilities;
                const count = Object.keys(caps).length;
                statCapabilities.textContent = String(count);
                renderCapabilities(caps);
            }

            // Run diagnostics
            const diagData = runDiagnostics(messages.value);
            if (diagData) {
                renderDiagnostics(diagData);
            }

            // Switch to diagnostics tab
            const diagTab = document.querySelector('[data-tab="diagnostics"]') as HTMLButtonElement;
            if (diagTab) diagTab.click();
        }
    } catch (e) {
        console.error('Analysis failed:', e);
        showError('Analysis failed: ' + String(e));
    }

    isAnalyzing.value = false;
    btnRunAnalysis.textContent = 'Run Analysis';
    btnRunAnalysis.disabled = false;
}

// ============================================================================
// Renderers
// ============================================================================

function renderTimeline() {
    if (!timelineContainer) return;

    const msgs = messages.value;
    if (msgs.length === 0) {
        timelineContainer.innerHTML = `
            <div class="empty-state">
                <p>📂 No messages loaded</p>
                <p class="hint">Upload a trace or connect to a charger</p>
            </div>
        `;
        return;
    }

    let html = '<div class="timeline">';
    for (let i = 0; i < msgs.length; i++) {
        const msg = msgs[i];
        const selected = selectedMessageIndex.value === i ? 'selected' : '';
        const isError = msg.message.includes('Error') || msg.message.includes('Faulted');
        const status = isError ? '❌' : '✅';
        const dirClass = msg.direction === 'incoming' ? 'incoming' : 'outgoing';
        const dirSymbol = msg.direction === 'incoming' ? '⬅️' : '➡️';

        html += `
            <div class="message-item ${selected}" data-index="${i}">
                <span class="time">${formatTime(msg.timestamp)}</span>
                <span class="direction ${dirClass}">${dirSymbol}</span>
                <span class="type">${escapeHtml(msg.message)}</span>
                <span class="status ${isError ? 'error' : 'success'}">${status}</span>
            </div>
        `;
    }
    html += '</div>';

    timelineContainer.innerHTML = html;

    // Click handler
    timelineContainer.querySelectorAll('.message-item').forEach((el) => {
        el.addEventListener('click', () => {
            const idx = parseInt(el.getAttribute('data-index') || '0', 10);
            selectedMessageIndex.value = idx;
            renderTimeline();
        });
    });
}

function renderStateMachine(state: any) {
    if (!stateContainer) return;

    const states = ['Initializing', 'Authorizing', 'Authorized', 'Charging', 'Suspended', 'Finishing', 'Completed', 'Faulted'];
    const current = state.state || 'Unknown';

    let html = `
        <div class="state-machine">
            <div class="state-current">${current}</div>
            <div class="state-transitions">
    `;

    for (const s of states) {
        const active = s === current ? 'active' : '';
        html += `<span class="state-node ${active}">${s}</span>`;
        if (s !== states[states.length - 1]) {
            html += `<span class="arrow">→</span>`;
        }
    }

    html += `
            </div>
            <div style="margin-top:16px;font-size:13px;color:var(--text-secondary);">
                Connectors: ${Object.keys(state.connectors || {}).length} active
                · Transactions: ${state.transactions || 0}
            </div>
        </div>
    `;

    stateContainer.innerHTML = html;
}

function renderCapabilities(caps: any) {
    if (!capabilitiesContainer) return;

    const entries = Object.entries(caps || {});
    if (entries.length === 0) {
        capabilitiesContainer.innerHTML = `
            <div class="empty-state">
                <p>🔧 No capabilities detected</p>
            </div>
        `;
        return;
    }

    let html = '<div class="capability-grid">';
    for (const [key, value] of entries) {
        const val = value as any;
        const supported = val && val.supported !== false;
        const limited = val && val.limited === true;
        const reason = val && val.reason ? val.reason : '';
        const cls = supported ? (limited ? 'limited' : 'supported') : 'unsupported';
        const statusText = supported ? (limited ? '⚠️' : '✅') : '❌';

        html += `
            <div class="capability-item ${cls}">
                <span class="name">${formatCapabilityName(key)}</span>
                <span class="status">${statusText}</span>
            </div>
        `;
    }
    html += '</div>';

    capabilitiesContainer.innerHTML = html;
}

function renderDiagnostics(diagData: any) {
    if (!diagnosticsContainer) return;

    const report = diagData.report || null;
    const hasIssues = diagData.has_issues || false;

    if (!report) {
        diagnosticsContainer.innerHTML = `
            <div class="empty-state">
                <p>🔍 No diagnostic data available</p>
            </div>
        `;
        return;
    }

    let html = '<div class="diagnostic-report">';

    // Summary
    const summaryClass = hasIssues ? 'has-issues' : '';
    html += `
        <div class="diagnostic-summary ${summaryClass}">
            <div class="title">${hasIssues ? '⚠️ Issues Detected' : '✅ No Issues Found'}</div>
            <div class="desc">${report.summary || 'Analysis complete'}</div>
        </div>
    `;

    // Root causes
    const rootCauses = report.root_causes || [];
    if (rootCauses.length > 0) {
        html += '<h3 style="color:var(--color-red);margin-bottom:12px;">🔍 Root Causes</h3>';
        for (const rc of rootCauses) {
            const confidence = Math.round((rc.confidence || 0) * 100);
            html += `
                <div class="root-cause">
                    <div class="title">${rc.title || 'Unknown Issue'}</div>
                    <div class="confidence">Confidence: ${confidence}%</div>
                    <div class="desc">${rc.description || ''}</div>
                    <ul class="causes">
            `;
            for (const cause of (rc.causes || [])) {
                const prob = Math.round((cause.probability || 0) * 100);
                html += `
                    <li>
                        ${cause.description || 'Unknown cause'}
                        <span class="prob">(${prob}%)</span>
                        <div class="mitigation">💡 ${cause.mitigation || 'No mitigation suggested'}</div>
                    </li>
                `;
            }
            html += '</ul></div>';
        }
    }

    html += '</div>';
    diagnosticsContainer.innerHTML = html;
}

// ============================================================================
// Error Display
// ============================================================================

function showError(message: string) {
    const container = document.getElementById('report');
    if (container) {
        container.innerHTML = `
            <div class="error-state" style="padding:40px;text-align:center;color:var(--color-red);">
                <p>❌ ${escapeHtml(message)}</p>
            </div>
        `;
    }
}

// ============================================================================
// Utilities
// ============================================================================

function formatTime(iso: string): string {
    try {
        const dt = new Date(iso);
        return dt.toTimeString().slice(0, 8);
    } catch {
        return iso.slice(11, 19);
    }
}

function formatCapabilityName(key: string): string {
    return key
        .replace(/_/g, ' ')
        .replace(/([A-Z])/g, ' $1')
        .trim()
        .split(' ')
        .map((w) => w.charAt(0).toUpperCase() + w.slice(1).toLowerCase())
        .join(' ');
}

function escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// ============================================================================
// Initialize
// ============================================================================

// Start WASM initialization
initWasm();

// Log startup
console.log('⚡ ChargeMesh Web Inspector started');