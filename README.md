# xmz-led-relais
Testprojekt xMZ-Mod-Touch LED und RELAIS Ansteuerung


## systemd Unit
### LED

Unit erstellen

    vim /etc/systemd/system/led.service


    [Unit]
    Description=LED Control
    After=multi-user.target

    [Service]
    Type=oneshot
    ExecStart=/root/enable_led1

    [Install]
    WantedBy=multi-user.target

    systemctl enable led.service
