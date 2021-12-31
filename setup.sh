sudo apt install firefox firefox-geckodriver
mkdir tontine && cd tontine
https://raw.githubusercontent.com/skyhawkb/tontine/main/tontine
chmod +x tontine
echo "EMAIL: $1" >> creds.txt
echo "PASSWORD: $2" >> creds.txt
