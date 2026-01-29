use anyhow::{Result, Context};
use lettre::{
    Message, SmtpTransport, Transport,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
};

pub struct EmailService {
    smtp_username: String,
    smtp_password: String,
    smtp_server: String,
    smtp_port: u16,
    from_email: String,
    from_name: String,
}

impl EmailService {
    pub fn from_env() -> Result<Self> {
        Ok(EmailService {
            smtp_username: std::env::var("SMTP_USERNAME")
                .context("SMTP_USERNAME not set in environment")?,
            smtp_password: std::env::var("SMTP_PASSWORD")
                .context("SMTP_PASSWORD not set in environment")?,
            smtp_server: std::env::var("SMTP_SERVER")
                .unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            from_email: std::env::var("FROM_EMAIL")
                .context("FROM_EMAIL not set in environment")?,
            from_name: std::env::var("FROM_NAME")
                .unwrap_or_else(|_| "Price Tracker".to_string()),
        })
    }

    pub async fn send_price_drop_alert(
        &self,
        to_email: &str,
        product_url: &str,
        current_price: f64,
        target_price: f64,
        platform: &str,
    ) -> Result<()> {
        let savings = target_price - current_price;
        let discount_percent = ((target_price - current_price) / target_price * 100.0).round();
        
        let subject = format!(
            "🚨 Price Drop Alert! Save ₹{:.0} on {}",
            savings,
            platform.to_uppercase()
        );
        
        let body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: linear-gradient(135deg, #6366f1, #ec4899); color: white; padding: 30px; text-align: center; border-radius: 12px 12px 0 0; }}
        .content {{ background: #f8f9fa; padding: 30px; }}
        .price-card {{ background: white; border-radius: 12px; padding: 25px; margin: 20px 0; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
        .price {{ font-size: 36px; font-weight: bold; color: #10b981; }}
        .old-price {{ text-decoration: line-through; color: #6b7280; font-size: 20px; }}
        .savings {{ background: #10b981; color: white; padding: 8px 16px; border-radius: 6px; display: inline-block; margin: 10px 0; }}
        .button {{ background: #6366f1; color: white; padding: 14px 28px; text-decoration: none; border-radius: 8px; display: inline-block; margin: 20px 0; font-weight: 600; }}
        .button:hover {{ background: #4f46e5; }}
        .platform {{ background: #ec4899; color: white; padding: 4px 12px; border-radius: 20px; font-size: 12px; font-weight: 600; }}
        .footer {{ text-align: center; padding: 20px; color: #6b7280; font-size: 14px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🎉 Price Drop Alert!</h1>
            <p>Your target price has been reached</p>
        </div>
        
        <div class="content">
            <div class="price-card">
                <span class="platform">{}</span>
                <h2>Great News!</h2>
                <p>The price has dropped below your target:</p>
                
                <div style="margin: 20px 0;">
                    <div class="old-price">Was: ₹{:.2}</div>
                    <div class="price">Now: ₹{:.2}</div>
                    <div class="savings">Save ₹{:.0} ({}% OFF)</div>
                </div>
                
                <p><strong>Product URL:</strong><br>
                <a href="{}" style="color: #6366f1; word-break: break-all;">{}</a></p>
                
                <a href="{}" class="button">🛍️ View Product Now</a>
            </div>
            
            <div style="background: #fff3cd; border-left: 4px solid #ffc107; padding: 15px; border-radius: 4px; margin: 20px 0;">
                <strong>⚡ Act Fast!</strong> Prices can change at any time. Don't miss this opportunity!
            </div>
        </div>
        
        <div class="footer">
            <p>This alert was sent because the price dropped to or below your target of ₹{:.2}</p>
            <p>You're receiving this because you set up a price alert at our service.</p>
            <p style="font-size: 12px; color: #9ca3af;">Clothing Price Tracker • Powered by Rust</p>
        </div>
    </div>
</body>
</html>"#,
            platform.to_uppercase(),
            target_price,
            current_price,
            savings,
            discount_percent,
            product_url,
            product_url,
            product_url,
            target_price
        );

        self.send_html_email(to_email, &subject, &body).await
    }

    async fn send_html_email(&self, to_email: &str, subject: &str, html_body: &str) -> Result<()> {
        let from_mailbox: Mailbox = format!("{} <{}>", self.from_name, self.from_email)
            .parse()
            .context("Invalid from email address")?;
        
        let to_mailbox: Mailbox = to_email
            .parse()
            .context("Invalid recipient email address")?;

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body.to_string())
            .context("Failed to build email message")?;

        let creds = Credentials::new(
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.smtp_server)
            .context("Failed to create SMTP transport")?
            .credentials(creds)
            .port(self.smtp_port)
            .build();

        // Send email in a blocking thread to avoid blocking the async runtime
        let result = tokio::task::spawn_blocking(move || mailer.send(&email))
            .await
            .context("Failed to spawn email sending task")?;

        result.context("Failed to send email")?;

        tracing::info!("📧 Email sent successfully to {}", to_email);
        Ok(())
    }

    pub async fn send_test_email(&self, to_email: &str) -> Result<()> {
        let subject = "✅ Price Tracker Email Setup Successful";
        let body = r#"<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #6366f1; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }
        .content { background: #f8f9fa; padding: 30px; border-radius: 0 0 8px 8px; }
        .success { background: #10b981; color: white; padding: 15px; border-radius: 6px; text-align: center; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🎉 Email Setup Complete!</h1>
        </div>
        <div class="content">
            <div class="success">
                <strong>✓ Your email notifications are working!</strong>
            </div>
            <p>This is a test email from your Clothing Price Tracker.</p>
            <p>You'll receive notifications at this email address when prices drop below your target.</p>
            <p><strong>What's next?</strong></p>
            <ul>
                <li>Create price alerts for your favorite products</li>
                <li>Set your target prices</li>
                <li>We'll monitor prices every 6 hours</li>
                <li>Get notified instantly when prices drop</li>
            </ul>
            <p style="color: #6b7280; font-size: 14px; margin-top: 30px;">
                Powered by Rust • Built with ❤️
            </p>
        </div>
    </div>
</body>
</html>"#;

        self.send_html_email(to_email, subject, body).await
    }
}
