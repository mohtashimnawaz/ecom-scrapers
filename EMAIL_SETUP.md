# Email Notifications Setup Guide

## ✨ Overview
The price tracker now sends **beautiful HTML email alerts** when product prices drop below your target price!

## 📧 Email Features
- **HTML Templates**: Professional-looking emails with product details
- **Price Drop Alerts**: Automatic notifications when prices fall
- **SMTP Support**: Works with Gmail, Outlook, custom SMTP servers
- **Background Worker**: Checks prices every 6 hours and sends alerts
- **Test Endpoint**: Verify email setup before going live

---

## 🚀 Quick Setup

### 1. Choose Your Email Provider

#### Option A: Gmail (Recommended for Testing)
1. Go to [Google App Passwords](https://myaccount.google.com/apppasswords)
2. Create a new app password for "Mail"
3. Copy the 16-character password

#### Option B: Custom SMTP Server
Use your own SMTP credentials (e.g., SendGrid, Mailgun, Amazon SES)

### 2. Update Environment Variables

Copy `.env.example` to `.env`:
```bash
cp .env.example .env
```

Edit `.env` with your SMTP credentials:
```dotenv
# Email Configuration
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-16-char-app-password
SMTP_SERVER=smtp.gmail.com
SMTP_PORT=587
FROM_EMAIL=your-email@gmail.com
FROM_NAME=Price Drop Alerts
```

**⚠️ For Gmail Users:**
- Use an **App Password**, not your regular password
- Enable 2-Factor Authentication first
- Generate the app password from Google Account settings

### 3. Start the Server

```bash
cargo run --release
```

The worker will automatically check prices every 6 hours and send email alerts!

---

## 🧪 Testing Email Setup

### Test Email Endpoint
```bash
./test_email.sh your-email@example.com
```

Or manually:
```bash
curl -X POST http://localhost:3000/email/test \
  -H "Content-Type: application/json" \
  -d '{"email": "your-email@example.com"}'
```

**Expected Response:**
```json
{
  "message": "Test email sent to your-email@example.com",
  "status": "success"
}
```

---

## 📨 Email Template

When a price drops, users receive an email like this:

```
Subject: 🎉 Price Drop Alert!

Hello!

Great news! The price has dropped on a product you're tracking:

🔗 Product URL: [Link to product]
💰 Current Price: ₹2,499
🎯 Your Target: ₹2,500
📉 You Save: ₹1

Don't miss out on this deal!

---
Price Tracker • Automated price monitoring for your favorite products
```

---

## 🔧 How It Works

### Background Worker
- Runs every **6 hours** automatically
- Checks all active price alerts
- Scrapes current product prices
- Compares with target prices
- Sends email if price dropped below target

### Email Flow
1. Worker detects price drop
2. `EmailService::send_price_drop_alert()` called
3. HTML email generated with product details
4. Sent via SMTP to user's email
5. Logs success/failure to console

---

## 📋 API Endpoints

### Test Email
```bash
POST /email/test
Content-Type: application/json

{
  "email": "test@example.com"
}
```

### Create Alert (with email)
```bash
POST /alerts
Content-Type: application/json

{
  "url": "https://www.myntra.com/...",
  "target_price": 2500,
  "user_email": "user@example.com"
}
```

---

## 🛠️ Troubleshooting

### Issue: "Email not configured" error
**Solution:** Check that all SMTP environment variables are set in `.env`

### Issue: Gmail authentication failed
**Solution:** 
- Use an **App Password**, not regular password
- Enable 2-Factor Authentication
- Generate new app password from [Google Account](https://myaccount.google.com/apppasswords)

### Issue: Emails not sending
**Solution:**
- Check server logs: `RUST_LOG=debug cargo run`
- Verify SMTP credentials
- Test with `./test_email.sh`
- Check spam folder

### Issue: Worker not running
**Solution:** Look for this log message:
```
🔄 Background worker started - checking prices every 6 hours
```

---

## 📊 Email Service Code

Located in: `src/email.rs`

**Key Functions:**
- `EmailService::from_env()` - Initialize from environment variables
- `send_price_drop_alert()` - Send price drop notification
- `send_test_email()` - Send test email to verify setup

**Dependencies:**
- `lettre` - SMTP email library
- `tokio` - Async runtime
- HTML templates embedded in code

---

## 🌟 Features

✅ Beautiful HTML emails  
✅ Automatic price monitoring  
✅ Gmail/custom SMTP support  
✅ Error handling & logging  
✅ Test endpoint for verification  
✅ Background worker (6-hour intervals)  
✅ Price drop calculations  
✅ Platform-specific scraping  

---

## 🔐 Security Notes

- **Never commit `.env`** to version control
- Use **App Passwords** for Gmail (not main password)
- Keep SMTP credentials secure
- `.env` is in `.gitignore` by default

---

## 📚 Next Steps

1. Configure SMTP credentials in `.env`
2. Test email with `./test_email.sh`
3. Create a price alert via API or frontend
4. Wait for price drop (or trigger manual check)
5. Receive email notification! 🎉

**Happy price tracking!** 🛍️
