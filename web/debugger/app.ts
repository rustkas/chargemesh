/**
 * ChargeMesh Inspector — Web Debugger
 * Uses WASM for protocol analysis
 */

// WASM module
import init, { DiagnosticEngine, ProtocolAnalyzer } from './wasm/chargemesh_wasm.js';

// State
let engine: any;
let analyzer: any;
let ws: WebSocket | null = null;

// Initialize
document.addEventListener('DOMContentLoaded', async () => {
    try {
        await init();
        engine = new DiagnosticEngine();
        analyzer = new ProtocolAnalyzer();
        console.log('✅ WASM initialized');
    } catch (e) {
        console.error('Failed to initialize WASM:', e);
        showError('Failed to load WASM module. Please refresh and try again.');
    }
    
    // Setup event listeners
    document.getElementById('upload-btn')?.addEventListener('click', handleUpload);
    document.getElementById('connect-btn')?.addEventListener('click', handleConnect);
});

// ============================================================================
// Handlers
// ============================================================================

async function handleUpload() {
    const fileInput = document.getElementById('file-input') as HTMLInputElement;
    if (!fileInput.files || fileInput.files.length === 0) {
        showError('Please select a file');
        return;
    }
    
    const file = fileInput.files[0];
    const content = await file.text();
    
    showStatus(`📊 Analyzing ${file.name}...`);
    
    try {
        const entries = parseOcppTrace(content);
        const report = engine.analyze(entries);
        renderReport(report);
        showStatus(`✅ Analysis complete: ${entries.length} messages processed`);
    } catch (e) {
        showError(`Failed to analyze: ${e}`);
    }
}

async function handleConnect() {
    const urlInput = document.getElementById('ws-url') as HTMLInputElement;
    if (!urlInput.value) {
        showError('Please enter a WebSocket URL');
        return;
    }
    
    if (ws) {
        ws.close();
        ws = null;
    }
    
    try {
        ws = new WebSocket(urlInput.value);
        
        ws.onopen = () => {
            showStatus('✅ Connected to charger. Capturing traffic...');
            document.getElementById('connect-btn')!.textContent = 'Disconnect';
        };
        
        ws.onmessage = (event) => {
            try {
                const result = analyzer.analyzeMessage(event.data);
                // Update report incrementally
                updateLiveReport(result);
            } catch (e) {
                console.warn('Failed to analyze message:', e);
            }
        };
        
        ws.onerror = (error) => {
            showError('WebSocket error');
            console.error(error);
        };
        
        ws.onclose = () => {
            showStatus('🔌 Disconnected');
            document.getElementById('connect-btn')!.textContent = 'Connect';
            ws = null;
        };
    } catch (e) {
        showError(`Failed to connect: ${e}`);
    }
}

// ============================================================================
// Parsing
// ============================================================================

function parseOcppTrace(content: string): any[] {
    const lines = content.split('\n');
    const entries: any[] = [];
    
    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed) continue;
        
        try {
            // Try to parse as JSON
            const parsed = JSON.parse(trimmed);
            entries.push({
                timestamp: new Date().toISOString(),
                direction: parsed.direction || 'unknown',
                message_type: parsed.action || parsed.message_type || 'unknown',
                payload: parsed.payload || parsed,
            });
        } catch {
            // Try OCPP array format
            try {
                const parts = JSON.parse(trimmed);
                if (Array.isArray(parts)) {
                    const msgType = parts[0];
                    let action = 'unknown';
                    let payload = {};
                    
                    if (msgType === 2 && parts.length >= 4) {
                        action = parts[2] || 'unknown';
                        payload = parts[3] || {};
                    } else if (msgType === 3 && parts.length >= 3) {
                        action = 'CallResult';
                        payload = parts[2] || {};
                    } else if (msgType === 4 && parts.length >= 5) {
                        action = `CallError: ${parts[2]}`;
                        payload = { error: parts[3] };
                    }
                    
                    entries.push({
                        timestamp: new Date().toISOString(),
                        direction: msgType === 2 ? 'outgoing' : 'incoming',
                        message_type: action,
                        payload,
                    });
                }
            } catch {
                // Skip
            }
        }
    }
    
    return entries;
}

// ============================================================================
// Rendering
// ============================================================================

function renderReport(report: any) {
    const container = document.getElementById('report');
    if (!container) return;
    
    let html = '<div class="report">';
    
    // Header
    html += `<h2>🔍 Diagnosis Report</h2>`;
    
    // Summary
    html += `<div class="summary ${report.summary.includes('⚠') ? 'warning' : 'success'}">`;
    html += `<p>${report.summary}</p>`;
    html += `</div>`;
    
    // Station info
    if (report.station) {
        html += `<div class="station-info">`;
        html += `<h3>📋 Station</h3>`;
        html += `<p><strong>Vendor:</strong> ${report.station.vendor}</p>`;
        html += `<p><strong>Model:</strong> ${report.station.model}</p>`;
        html += `<p><strong>Status:</strong> ${report.station.state || 'unknown'}</p>`;
        html += `</div>`;
    }
    
    // Errors
    if (report.errors && report.errors.length > 0) {
        html += `<div class="errors">`;
        html += `<h3>❌ Errors (${report.errors.length})</h3>`;
        for (const error of report.errors) {
            html += `<div class="error-item">`;
            html += `<p><strong>${error.error_code}</strong>: ${error.description}</p>`;
            if (error.root_cause) {
                html += `<div class="root-cause">💡 ${error.root_cause}</div>`;
            }
            html += `</div>`;
        }
        html += `</div>`;
    }
    
    // Violations
    if (report.violations && report.violations.length > 0) {
        html += `<div class="violations">`;
        html += `<h3>⚠️ State Violations (${report.violations.length})</h3>`;
        for (const v of report.violations) {
            html += `<div class="violation-item">`;
            html += `<p><strong>${v.message}</strong></p>`;
            html += `<p class="detail">Expected: ${v.expected_state} → Actual: ${v.actual_state}</p>`;
            html += `</div>`;
        }
        html += `</div>`;
    }
    
    // Recommendations
    if (report.recommendations && report.recommendations.length > 0) {
        html += `<div class="recommendations">`;
        html += `<h3>💡 Recommendations</h3>`;
        for (const rec of report.recommendations) {
            html += `<div class="rec-item">`;
            html += `<p><strong>${rec.action}</strong></p>`;
            html += `<p>${rec.description}</p>`;
            html += `</div>`;
        }
        html += `</div>`;
    }
    
    html += '</div>';
    container.innerHTML = html;
}

function updateLiveReport(result: any) {
    // Incrementally update the report
    // For now, just log
    console.log('Live update:', result);
}

function showStatus(message: string) {
    const container = document.getElementById('report');
    if (container) {
        container.innerHTML = `<div class="status">${message}</div>`;
    }
}

function showError(message: string) {
    const container = document.getElementById('report');
    if (container) {
        container.innerHTML = `<div class="error">❌ ${message}</div>`;
    }
}
