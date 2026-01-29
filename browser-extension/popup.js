// Popup Script - Extension popup interface

const API_BASE = 'http://localhost:3000';

let authToken = null;
let currentUser = null;
let alerts = [];

// Initialize popup
document.addEventListener('DOMContentLoaded', async () => {
  setupEventListeners();
  await checkAuth();
});

// Setup event listeners
function setupEventListeners() {
  document.getElementById('loginBtn').addEventListener('click', handleLogin);
  document.getElementById('signupBtn').addEventListener('click', handleSignup);
  document.getElementById('logoutBtn').addEventListener('click', handleLogout);
  document.getElementById('refreshBtn').addEventListener('click', loadAlerts);
  document.getElementById('openDashboard').addEventListener('click', openDashboard);
  
  document.getElementById('showSignup').addEventListener('click', (e) => {
    e.preventDefault();
    document.getElementById('loginForm').style.display = 'none';
    document.getElementById('signupForm').style.display = 'block';
  });
  
  document.getElementById('showLogin').addEventListener('click', (e) => {
    e.preventDefault();
    document.getElementById('signupForm').style.display = 'none';
    document.getElementById('loginForm').style.display = 'block';
  });
}

// Check authentication
async function checkAuth() {
  const storage = await chrome.storage.local.get(['authToken', 'userEmail']);
  
  if (storage.authToken) {
    authToken = storage.authToken;
    
    // Verify token
    try {
      const response = await fetch(`${API_BASE}/auth/me`, {
        headers: { 'Authorization': `Bearer ${authToken}` }
      });
      
      if (response.ok) {
        currentUser = await response.json();
        showAppSection();
        await loadAlerts();
      } else {
        // Token invalid
        await chrome.storage.local.remove(['authToken', 'userEmail']);
        showAuthSection();
      }
    } catch (error) {
      console.error('Auth check failed:', error);
      showAuthSection();
    }
  } else {
    showAuthSection();
  }
}

// Handle login
async function handleLogin() {
  const email = document.getElementById('loginEmail').value;
  const password = document.getElementById('loginPassword').value;
  
  if (!email || !password) {
    showToast('Please enter email and password', 'error');
    return;
  }
  
  try {
    const response = await fetch(`${API_BASE}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password })
    });
    
    if (response.ok) {
      const data = await response.json();
      authToken = data.token;
      currentUser = data.user;
      
      await chrome.storage.local.set({
        authToken: data.token,
        userEmail: data.user.email
      });
      
      showToast('Login successful!', 'success');
      showAppSection();
      await loadAlerts();
    } else {
      const error = await response.text();
      showToast(error || 'Login failed', 'error');
    }
  } catch (error) {
    console.error('Login error:', error);
    showToast('Login failed', 'error');
  }
}

// Handle signup
async function handleSignup() {
  const email = document.getElementById('signupEmail').value;
  const password = document.getElementById('signupPassword').value;
  
  if (!email || !password) {
    showToast('Please enter email and password', 'error');
    return;
  }
  
  if (password.length < 6) {
    showToast('Password must be at least 6 characters', 'error');
    return;
  }
  
  try {
    const response = await fetch(`${API_BASE}/auth/signup`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password })
    });
    
    if (response.ok) {
      const data = await response.json();
      authToken = data.token;
      currentUser = data.user;
      
      await chrome.storage.local.set({
        authToken: data.token,
        userEmail: data.user.email
      });
      
      showToast('Account created!', 'success');
      showAppSection();
      await loadAlerts();
    } else {
      const error = await response.text();
      showToast(error || 'Signup failed', 'error');
    }
  } catch (error) {
    console.error('Signup error:', error);
    showToast('Signup failed', 'error');
  }
}

// Handle logout
async function handleLogout() {
  await chrome.storage.local.remove(['authToken', 'userEmail']);
  authToken = null;
  currentUser = null;
  alerts = [];
  showAuthSection();
  showToast('Logged out', 'success');
}

// Load alerts
async function loadAlerts() {
  const listContainer = document.getElementById('alertsList');
  listContainer.innerHTML = '<div class="loading">Loading...</div>';
  
  try {
    const response = await fetch(`${API_BASE}/alerts`, {
      headers: { 'Authorization': `Bearer ${authToken}` }
    });
    
    if (response.ok) {
      alerts = await response.json();
      renderAlerts();
      updateStats();
    } else {
      throw new Error('Failed to load alerts');
    }
  } catch (error) {
    console.error('Error loading alerts:', error);
    listContainer.innerHTML = '<div class="empty-state">Failed to load alerts</div>';
  }
}

// Render alerts
function renderAlerts() {
  const listContainer = document.getElementById('alertsList');
  
  if (alerts.length === 0) {
    listContainer.innerHTML = '<div class="empty-state">No alerts yet.<br>Visit a product page and click "Track Price"</div>';
    return;
  }
  
  listContainer.innerHTML = alerts.map(alert => {
    const isPriceDrop = alert.last_price && alert.last_price <= alert.target_price;
    
    return `
      <div class="alert-item ${isPriceDrop ? 'price-drop' : ''}">
        <span class="alert-platform">${alert.platform}</span>
        <div class="alert-url" title="${alert.url}">${truncateUrl(alert.url)}</div>
        <div class="alert-prices">
          <div class="price">
            <div class="price-label">Target</div>
            <div class="price-value">₹${alert.target_price.toFixed(0)}</div>
          </div>
          <div class="price">
            <div class="price-label">Current</div>
            <div class="price-value ${isPriceDrop ? 'drop' : ''}">
              ${alert.last_price ? '₹' + alert.last_price.toFixed(0) : 'N/A'}
            </div>
          </div>
        </div>
        <div class="alert-actions">
          <button class="btn btn-delete" data-id="${alert.id}">Delete</button>
        </div>
      </div>
    `;
  }).join('');
  
  // Attach delete handlers
  document.querySelectorAll('.btn-delete').forEach(btn => {
    btn.addEventListener('click', () => handleDeleteAlert(btn.dataset.id));
  });
}

// Update stats
function updateStats() {
  const priceDrops = alerts.filter(a => a.last_price && a.last_price <= a.target_price).length;
  
  document.getElementById('alertCount').textContent = alerts.length;
  document.getElementById('priceDrops').textContent = priceDrops;
  
  // Update badge
  chrome.storage.local.set({ priceDrops });
}

// Delete alert
async function handleDeleteAlert(id) {
  if (!confirm('Delete this alert?')) return;
  
  try {
    const response = await fetch(`${API_BASE}/alerts/${id}`, {
      method: 'DELETE',
      headers: { 'Authorization': `Bearer ${authToken}` }
    });
    
    if (response.ok) {
      showToast('Alert deleted', 'success');
      await loadAlerts();
    } else {
      throw new Error('Failed to delete');
    }
  } catch (error) {
    console.error('Delete error:', error);
    showToast('Failed to delete alert', 'error');
  }
}

// Show auth section
function showAuthSection() {
  document.getElementById('authSection').style.display = 'block';
  document.getElementById('appSection').style.display = 'none';
}

// Show app section
function showAppSection() {
  document.getElementById('authSection').style.display = 'none';
  document.getElementById('appSection').style.display = 'block';
  document.getElementById('userEmail').textContent = currentUser.email;
}

// Open dashboard
function openDashboard() {
  chrome.tabs.create({ url: `${API_BASE}/app/` });
}

// Show toast notification
function showToast(message, type = 'info') {
  const toast = document.getElementById('toast');
  toast.textContent = message;
  toast.className = `toast ${type} show`;
  
  setTimeout(() => {
    toast.classList.remove('show');
  }, 3000);
}

// Truncate URL
function truncateUrl(url) {
  const maxLength = 40;
  if (url.length <= maxLength) return url;
  return url.substring(0, maxLength) + '...';
}
