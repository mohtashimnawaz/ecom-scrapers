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
        
        // Load price history for each alert
        loadPriceHistory(alert.id);
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
            
            <!-- Price History Chart -->
            <div class="price-history-section">
                <div class="chart-header">
                    <h3>📊 Price History</h3>
                    <button class="btn-link" onclick="loadPriceHistory('${alert.id}')">View Details</button>
                </div>
                <div id="stats-${alert.id}" class="price-stats"></div>
                <canvas id="chart-${alert.id}" class="price-chart" height="80"></canvas>
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

// Price History Functions
async function loadPriceHistory(alertId) {
    try {
        // Load price stats
        const statsResponse = await fetch(`${API_BASE}/alerts/${alertId}/stats`);
        if (statsResponse.ok) {
            const stats = await statsResponse.json();
            displayPriceStats(alertId, stats);
        }
        
        // Load price history data
        const historyResponse = await fetch(`${API_BASE}/alerts/${alertId}/history`);
        if (historyResponse.ok) {
            const data = await historyResponse.json();
            if (data.history && data.history.length > 0) {
                renderPriceChart(alertId, data.history);
            }
        }
    } catch (error) {
        console.error('Error loading price history:', error);
    }
}

function displayPriceStats(alertId, stats) {
    const container = document.getElementById(`stats-${alertId}`);
    if (!container) return;
    
    if (stats.data_points && stats.data_points > 0) {
        container.innerHTML = `
            <div class="stats-grid">
                <div class="stat-item">
                    <span class="stat-label">Best Price:</span>
                    <span class="stat-value best">₹${stats.lowest_price.toFixed(2)}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">Highest:</span>
                    <span class="stat-value">₹${stats.highest_price.toFixed(2)}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">Average:</span>
                    <span class="stat-value">₹${stats.average_price.toFixed(2)}</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">Checks:</span>
                    <span class="stat-value">${stats.data_points}</span>
                </div>
            </div>
        `;
    } else {
        container.innerHTML = '<p class="no-data">No price history yet</p>';
    }
}

function renderPriceChart(alertId, history) {
    const canvas = document.getElementById(`chart-${alertId}`);
    if (!canvas) return;
    
    // Reverse to show oldest to newest
    const sortedHistory = [...history].reverse();
    
    const labels = sortedHistory.map(h => {
        const date = new Date(h.checked_at);
        return date.toLocaleDateString('en-IN', { month: 'short', day: 'numeric' });
    });
    
    const prices = sortedHistory.map(h => h.price);
    
    // Destroy existing chart if it exists
    const existingChart = Chart.getChart(canvas);
    if (existingChart) {
        existingChart.destroy();
    }
    
    new Chart(canvas, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [{
                label: 'Price (₹)',
                data: prices,
                borderColor: '#10b981',
                backgroundColor: 'rgba(16, 185, 129, 0.1)',
                tension: 0.4,
                fill: true,
                pointRadius: 4,
                pointHoverRadius: 6
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    display: false
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    backgroundColor: 'rgba(0, 0, 0, 0.8)',
                    padding: 12,
                    titleColor: '#fff',
                    bodyColor: '#fff',
                    callbacks: {
                        label: function(context) {
                            return 'Price: ₹' + context.parsed.y.toFixed(2);
                        }
                    }
                }
            },
            scales: {
                y: {
                    beginAtZero: false,
                    ticks: {
                        color: '#9ca3af',
                        callback: function(value) {
                            return '₹' + value;
                        }
                    },
                    grid: {
                        color: 'rgba(75, 85, 99, 0.2)'
                    }
                },
                x: {
                    ticks: {
                        color: '#9ca3af',
                        maxRotation: 45,
                        minRotation: 0
                    },
                    grid: {
                        color: 'rgba(75, 85, 99, 0.2)'
                    }
                }
            }
        }
    });
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
