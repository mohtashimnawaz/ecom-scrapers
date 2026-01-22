const API_BASE = 'http://localhost:3000';

// State
let alerts = [];
let autoRefreshInterval = null;

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    loadAlerts();
    setupEventListeners();
    startAutoRefresh();
});

// Event Listeners
function setupEventListeners() {
    document.getElementById('alertForm').addEventListener('submit', handleCreateAlert);
    document.getElementById('refreshBtn').addEventListener('click', loadAlerts);
    document.getElementById('checkPricesBtn').addEventListener('click', handleCheckPrices);
}

// API Calls
async function loadAlerts() {
    try {
        showLoading();
        const response = await fetch(`${API_BASE}/alerts`);
        
        if (!response.ok) {
            throw new Error('Failed to fetch alerts');
        }
        
        alerts = await response.json();
        renderAlerts();
        updateStats();
    } catch (error) {
        console.error('Error loading alerts:', error);
        showToast('Failed to load alerts', 'error');
        showEmptyState('Error loading alerts. Make sure the server is running.');
    }
}

async function handleCreateAlert(e) {
    e.preventDefault();
    
    const formData = new FormData(e.target);
    const data = {
        url: formData.get('url'),
        target_price: parseFloat(formData.get('target_price')),
        user_email: formData.get('user_email')
    };
    
    try {
        const response = await fetch(`${API_BASE}/alerts`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data)
        });
        
        if (!response.ok) {
            const error = await response.text();
            throw new Error(error || 'Failed to create alert');
        }
        
        const newAlert = await response.json();
        showToast('✅ Alert created successfully!', 'success');
        e.target.reset();
        loadAlerts();
    } catch (error) {
        console.error('Error creating alert:', error);
        showToast(`Failed to create alert: ${error.message}`, 'error');
    }
}

async function handleDeleteAlert(id) {
    if (!confirm('Are you sure you want to delete this alert?')) {
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/alerts/${id}`, {
            method: 'DELETE'
        });
        
        if (!response.ok) {
            throw new Error('Failed to delete alert');
        }
        
        showToast('🗑️ Alert deleted successfully', 'success');
        loadAlerts();
    } catch (error) {
        console.error('Error deleting alert:', error);
        showToast('Failed to delete alert', 'error');
    }
}

async function handleCheckPrices() {
    try {
        showToast('⏳ Checking prices...', 'info');
        const response = await fetch(`${API_BASE}/alerts/check`, {
            method: 'POST'
        });
        
        if (!response.ok) {
            throw new Error('Failed to check prices');
        }
        
        const result = await response.json();
        showToast('✅ Price check completed!', 'success');
        
        // Reload alerts after check
        setTimeout(loadAlerts, 1000);
    } catch (error) {
        console.error('Error checking prices:', error);
        showToast('Failed to check prices', 'error');
    }
}

// Rendering
function renderAlerts() {
    const container = document.getElementById('alertsList');
    
    if (alerts.length === 0) {
        showEmptyState();
        return;
    }
    
    const html = alerts.map(alert => createAlertCard(alert)).join('');
    container.innerHTML = html;
    
    // Attach delete handlers
    alerts.forEach(alert => {
        const deleteBtn = document.querySelector(`[data-delete-id="${alert.id}"]`);
        if (deleteBtn) {
            deleteBtn.addEventListener('click', () => handleDeleteAlert(alert.id));
        }
    });
}

function createAlertCard(alert) {
    const currentPrice = alert.last_price || 'Not checked yet';
    const isPriceDrop = alert.last_price && alert.last_price <= alert.target_price;
    const priceClass = isPriceDrop ? 'price-drop' : 'price-current';
    
    return `
        <div class="alert-item ${isPriceDrop ? 'price-drop-highlight' : ''}">
            <div class="alert-header">
                <span class="platform-badge platform-${alert.platform}">${alert.platform}</span>
                <button class="btn btn-danger" data-delete-id="${alert.id}">Delete</button>
            </div>
            
            <div class="alert-url">${truncateUrl(alert.url)}</div>
            
            <div class="alert-prices">
                <div class="price-info">
                    <div class="price-label">Target Price</div>
                    <div class="price-value price-target">₹${alert.target_price.toFixed(2)}</div>
                </div>
                
                <div class="price-info">
                    <div class="price-label">Current Price</div>
                    <div class="price-value ${priceClass}">
                        ${typeof currentPrice === 'number' ? '₹' + currentPrice.toFixed(2) : currentPrice}
                    </div>
                </div>
                
                ${alert.last_price ? `
                    <div class="price-info">
                        <div class="price-label">Savings</div>
                        <div class="price-value ${isPriceDrop ? 'price-drop' : ''}">
                            ${alert.last_price <= alert.target_price 
                                ? '🎉 ₹' + (alert.target_price - alert.last_price).toFixed(2)
                                : '₹' + (alert.last_price - alert.target_price).toFixed(2) + ' away'
                            }
                        </div>
                    </div>
                ` : ''}
            </div>
            
            <div class="alert-meta">
                <div class="alert-email">📧 ${alert.user_email}</div>
                ${isPriceDrop ? '<div class="price-drop">🚨 PRICE DROP DETECTED!</div>' : ''}
            </div>
        </div>
    `;
}

function showLoading() {
    document.getElementById('alertsList').innerHTML = '<div class="loading">Loading alerts...</div>';
}

function showEmptyState(message = 'No active alerts. Create your first one above!') {
    document.getElementById('alertsList').innerHTML = `
        <div class="empty-state">
            <div class="empty-state-icon">🛍️</div>
            <p>${message}</p>
        </div>
    `;
}

function updateStats() {
    const totalAlerts = alerts.length;
    const priceDrops = alerts.filter(a => a.last_price && a.last_price <= a.target_price).length;
    const avgTargetPrice = alerts.length > 0 
        ? alerts.reduce((sum, a) => sum + a.target_price, 0) / alerts.length
        : 0;
    
    document.getElementById('totalAlerts').textContent = totalAlerts;
    document.getElementById('priceDrops').textContent = priceDrops;
    document.getElementById('avgSavings').textContent = '₹' + avgTargetPrice.toFixed(0);
}

// Utilities
function truncateUrl(url, maxLength = 60) {
    if (url.length <= maxLength) return url;
    return url.substring(0, maxLength) + '...';
}

function showToast(message, type = 'info') {
    const container = document.getElementById('toastContainer');
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    
    container.appendChild(toast);
    
    setTimeout(() => {
        toast.style.animation = 'slideIn 0.3s ease-out reverse';
        setTimeout(() => toast.remove(), 300);
    }, 3000);
}

function startAutoRefresh() {
    // Refresh alerts every 30 seconds
    autoRefreshInterval = setInterval(loadAlerts, 30000);
}

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
    if (autoRefreshInterval) {
        clearInterval(autoRefreshInterval);
    }
});
