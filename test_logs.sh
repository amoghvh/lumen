#!/bin/bash
echo "Sending test logs to Lumen on port 9999..."
echo "User amogh logged in" > /dev/tcp/127.0.0.1/9999
echo "API request to /users" > /dev/tcp/127.0.0.1/9999
echo "SELECT * FROM users" > /dev/tcp/127.0.0.1/9999
echo "<script>alert('xss')</script>" > /dev/tcp/127.0.0.1/9999
echo "cc_number=4111111111111111" > /dev/tcp/127.0.0.1/9999
echo "Done! Check Lumen terminal."
