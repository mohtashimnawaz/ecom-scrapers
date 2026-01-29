// Background Service Worker
// Handles API communication and storage

const API_BASE = 'http://localhost:3000';

// Listen for messages from content script and popup
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.action === 'createAlert') {
    createAlert(request.data)
      .then(sendResponse)
      .catch(error => sendResponse({ error: error.message }));
    return true; // Keep channel open for async response
  }
  
  if (request.action === 'getAlerts') {
    getAlerts(request.token)
      .then(sendResponse)
      .catch(error => sendResponse({ error: error.message }));
    return true;
  }
  
  if (request.action === 'deleteAlert') {
    deleteAlert(request.id, request.token)
      .then(sendResponse)
      .catch(error => sendResponse({ error: error.message }));
    return true;
  }
});

// Create price alert
async function createAlert(data) {
  const { url, targetPrice, userEmail, token } = data;
  
  const response = await fetch(`${API_BASE}/alerts`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`
    },
    body: JSON.stringify({
      url,
      target_price: targetPrice,
      user_email: userEmail
    })
  });
  
  if (!response.ok) {
    const error = await response.text();
    throw new Error(error);
  }
  
  return await response.json();
}

// Get user's alerts
async function getAlerts(token) {
  const response = await fetch(`${API_BASE}/alerts`, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  
  if (!response.ok) {
    throw new Error('Failed to fetch alerts');
  }
  
  return await response.json();
}

// Delete alert
async function deleteAlert(id, token) {
  const response = await fetch(`${API_BASE}/alerts/${id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  
  if (!response.ok) {
    throw new Error('Failed to delete alert');
  }
  
  return { success: true };
}

// Badge notification for price drops
chrome.storage.onChanged.addListener((changes, namespace) => {
  if (changes.priceDrops) {
    const count = changes.priceDrops.newValue || 0;
    if (count > 0) {
      chrome.action.setBadgeText({ text: String(count) });
      chrome.action.setBadgeBackgroundColor({ color: '#ef4444' });
    } else {
      chrome.action.setBadgeText({ text: '' });
    }
  }
});
