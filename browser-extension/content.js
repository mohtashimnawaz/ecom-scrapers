// Content Script - Runs on product pages
// Detects platform and adds "Track This Price" button

const API_BASE = 'http://localhost:3000';

// Platform detection patterns
const PLATFORMS = {
  myntra: {
    pattern: /myntra\.com/i,
    selector: '.pdp-price',
    urlPattern: /myntra\.com\/([^\/]+)\/([^\/]+)\/(\d+)/
  },
  flipkart: {
    pattern: /flipkart\.com/i,
    selector: '._30jeq3, ._25b18c',
    urlPattern: /flipkart\.com\/.*\/p\//
  },
  ajio: {
    pattern: /ajio\.com/i,
    selector: '.prod-sp',
    urlPattern: /ajio\.com\/.*\/p\//
  },
  tatacliq: {
    pattern: /tatacliq\.com/i,
    selector: '.ProductDescription__price',
    urlPattern: /tatacliq\.com\/.*\/p-/
  }
};

// Detect current platform
function detectPlatform() {
  const url = window.location.href;
  
  for (const [name, config] of Object.entries(PLATFORMS)) {
    if (config.pattern.test(url) && config.urlPattern.test(url)) {
      return { name, config };
    }
  }
  
  return null;
}

// Extract price from page
function extractPrice(selector) {
  const priceElement = document.querySelector(selector);
  if (!priceElement) return null;
  
  const priceText = priceElement.textContent || priceElement.innerText;
  const priceMatch = priceText.match(/[\d,]+/);
  
  if (priceMatch) {
    return parseFloat(priceMatch[0].replace(/,/g, ''));
  }
  
  return null;
}

// Create and inject tracking button
function createTrackButton(platform, currentPrice) {
  // Check if button already exists
  if (document.getElementById('price-tracker-btn')) {
    return;
  }
  
  const button = document.createElement('div');
  button.id = 'price-tracker-btn';
  button.className = 'price-tracker-button';
  button.innerHTML = `
    <div class="tracker-btn-content">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"></path>
      </svg>
      <span>Track Price</span>
    </div>
  `;
  
  button.addEventListener('click', () => handleTrackPrice(platform, currentPrice));
  
  // Insert button based on platform
  const priceContainer = document.querySelector(platform.config.selector);
  if (priceContainer) {
    const parent = priceContainer.closest('.pdp-price-info, .product-actions, .prod-price');
    if (parent) {
      parent.insertAdjacentElement('afterend', button);
    } else {
      priceContainer.parentElement.insertAdjacentElement('afterend', button);
    }
  } else {
    // Fallback: add to body with fixed position
    button.style.position = 'fixed';
    button.style.bottom = '20px';
    button.style.right = '20px';
    button.style.zIndex = '999999';
    document.body.appendChild(button);
  }
}

// Handle track price click
async function handleTrackPrice(platform, currentPrice) {
  const button = document.getElementById('price-tracker-btn');
  
  // Get auth token
  const result = await chrome.storage.local.get(['authToken', 'userEmail']);
  
  if (!result.authToken) {
    showNotification('Please login first', 'Click the extension icon to login', 'error');
    return;
  }
  
  // Show loading state
  button.classList.add('loading');
  button.innerHTML = '<span>Adding...</span>';
  
  // Get target price from user
  const targetPrice = prompt(`Current price: ₹${currentPrice}\n\nEnter your target price (₹):`);
  
  if (!targetPrice || isNaN(targetPrice)) {
    resetButton(button);
    return;
  }
  
  try {
    const response = await fetch(`${API_BASE}/alerts`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${result.authToken}`
      },
      body: JSON.stringify({
        url: window.location.href,
        target_price: parseFloat(targetPrice),
        user_email: result.userEmail
      })
    });
    
    if (response.ok) {
      button.classList.remove('loading');
      button.classList.add('success');
      button.innerHTML = `
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <path d="M20 6L9 17l-5-5"></path>
        </svg>
        <span>Tracking!</span>
      `;
      
      showNotification(
        'Price Alert Created!',
        `You'll be notified when price drops below ₹${targetPrice}`,
        'success'
      );
      
      setTimeout(() => resetButton(button), 3000);
    } else {
      throw new Error('Failed to create alert');
    }
  } catch (error) {
    console.error('Error creating alert:', error);
    showNotification('Failed to create alert', error.message, 'error');
    resetButton(button);
  }
}

// Reset button to original state
function resetButton(button) {
  button.classList.remove('loading', 'success');
  button.innerHTML = `
    <div class="tracker-btn-content">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"></path>
      </svg>
      <span>Track Price</span>
    </div>
  `;
}

// Show notification toast
function showNotification(title, message, type = 'info') {
  const notification = document.createElement('div');
  notification.className = `price-tracker-notification ${type}`;
  notification.innerHTML = `
    <div class="notification-content">
      <strong>${title}</strong>
      <p>${message}</p>
    </div>
  `;
  
  document.body.appendChild(notification);
  
  setTimeout(() => {
    notification.classList.add('show');
  }, 100);
  
  setTimeout(() => {
    notification.classList.remove('show');
    setTimeout(() => notification.remove(), 300);
  }, 4000);
}

// Initialize on page load
function init() {
  const platform = detectPlatform();
  
  if (platform) {
    console.log('Price Tracker: Detected platform:', platform.name);
    
    const price = extractPrice(platform.config.selector);
    
    if (price) {
      console.log('Price Tracker: Current price:', price);
      createTrackButton(platform, price);
    } else {
      console.log('Price Tracker: Could not extract price');
    }
  }
}

// Run on page load
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}

// Re-run for single-page apps
let lastUrl = location.href;
new MutationObserver(() => {
  const url = location.href;
  if (url !== lastUrl) {
    lastUrl = url;
    setTimeout(init, 1000); // Wait for page to update
  }
}).observe(document, { subtree: true, childList: true });
