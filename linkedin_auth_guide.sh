#!/usr/bin/env bash

echo "LinkedIn Authentication Troubleshooting Guide"
echo "============================================="
echo ""

echo "The LinkedIn scraper is working correctly, but LinkedIn has detected"
echo "automated access and is requiring additional verification."
echo ""

echo "Common scenarios and solutions:"
echo ""

echo "1. CHALLENGE PAGE (/challenge or /uas):"
echo "   - LinkedIn detected unusual login activity"
echo "   - Solution: Log in manually via web browser first"
echo "   - Complete any CAPTCHA or verification steps"
echo "   - Then try the CLI tool again"
echo ""

echo "2. WRONG CREDENTIALS:"
echo "   - Double-check your email and password"
echo "   - Make sure your account isn't locked"
echo "   - Try logging in via web browser to verify"
echo ""

echo "3. IP/LOCATION ISSUES:"
echo "   - Use the same network you normally use LinkedIn from"
echo "   - Avoid VPNs or proxy servers"
echo "   - Try again after some time"
echo ""

echo "4. ACCOUNT SECURITY:"
echo "   - LinkedIn may require email verification"
echo "   - Two-factor authentication needs to be handled manually"
echo "   - Check your email for LinkedIn security notifications"
echo ""

echo "Testing your authentication:"
echo "1. First, log in to LinkedIn manually at https://linkedin.com/login"
echo "2. Complete any verification steps"
echo "3. Keep that browser session open"
echo "4. Then try the CLI tool"
echo ""

echo "Environment variables (recommended method):"
echo "export LINKEDIN_EMAIL=\"your-email@example.com\""
echo "export LINKEDIN_PASSWORD=\"your-password\""
echo ""

echo "Test command:"
echo "./target/release/in_scraper --help"
echo ""

echo "If you continue having issues, the problem is likely:"
echo "- LinkedIn's anti-bot protection (very common)"
echo "- Need to complete manual verification first"
echo "- Account-specific security settings"
