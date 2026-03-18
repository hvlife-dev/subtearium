#!/usr/bin/bash 

echo ""
echo "===================================================================="
echo " This script uses hvlife/subtearium docker image as a source"
echo " You have to have lxc and docker installed"
echo " I've only checked it on Arch Linux"
echo "===================================================================="
read -p "Press [Enter] to acknowledge and proceed..."
echo ""

CT_NAME="leptos-runner"

if [[ $(/usr/bin/id -u) -ne 0 ]]; then
    echo "Error: Not running as root"
    exit 1
fi

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

if lxc-info -n "$CT_NAME" >/dev/null 2>&1; then
    echo "Error: An LXC container named '$CT_NAME' already exists."
    echo "Please remove it first (lxc-destroy -n $CT_NAME -f) or change CT_NAME in this script."
    exit 1
fi

echo "Creating temporary docker image clone for copy..."
docker create --name temp-extractor hvlife/subtearium:$1

mkdir -p .artifacts
docker cp temp-extractor:/app/subtearium .artifacts/
docker cp temp-extractor:/app/site .artifacts/
docker cp temp-extractor:/app/Cargo.toml .artifacts/

docker rm temp-extractor

echo ""
echo "===================================================================="
echo " ATTENTION: LXC Networking Setup Required "
echo " In my case, this meant putting USE_LXC_BRIDGE=\"true\" inside:"
echo " /etc/default/lxc-net"
echo "===================================================================="
read -p "Press [Enter] to acknowledge and proceed..."
echo ""

echo "Starting lxc-net..."
systemctl start lxc-net

echo "Creating lxc image..."
lxc-create -n $CT_NAME -t download -- -d alpine -r edge -a amd64
lxc-start -n $CT_NAME -d 

echo "Waiting for container network to initialize..."
sleep 5

lxc-attach -n $CT_NAME -- /sbin/apk update
lxc-attach -n $CT_NAME -- /sbin/apk add --no-cache tzdata libgcc
lxc-attach -n $CT_NAME -- /bin/mkdir -p /app

echo "Copying files to container..."
cp .artifacts/subtearium /var/lib/lxc/$CT_NAME/rootfs/app/
cp -r .artifacts/site /var/lib/lxc/$CT_NAME/rootfs/app/site
cp .artifacts/Cargo.toml /var/lib/lxc/$CT_NAME/rootfs/app/

echo "Creating openrc script..."
cat << 'EOF' > /var/lib/lxc/$CT_NAME/rootfs/etc/init.d/subtearium
#!/sbin/openrc-run

name="subtearium"
description="Subtearium"
command="/app/subtearium"
command_background="yes"
pidfile="/run/${name}.pid"
directory="/app"

export RUST_LOG="info"
export LEPTOS_SITE_ADDR="0.0.0.0:2137"
export LEPTOS_SITE_ROOT="./site"

output_log="/var/log/subtearium.log"
error_log="/var/log/subtearium.err"

depend() {
    need net
}
EOF

chmod +x /var/lib/lxc/$CT_NAME/rootfs/etc/init.d/subtearium

echo "Enabling and starting service..."
lxc-attach -n $CT_NAME -- /sbin/rc-update add subtearium default
lxc-attach -n $CT_NAME -- /sbin/rc-service subtearium start

lxc-ls -f

echo ""
echo "===================================================================="
echo " Deployment to container complete! "
echo "===================================================================="
echo ""

WORK_DIR=$(pwd)

read -p "Would you like to compress the LXC container for sharing? [y/N]: " COMPRESS_CHOICE

if [[ "$COMPRESS_CHOICE" =~ ^[Yy]$ ]]; then
    echo "Stopping the container to safely create the archive..."
    lxc-stop -n $CT_NAME

    echo "Compressing container to .artifacts/${CT_NAME}-export.tar.gz..."
    tar -czvf "${WORK_DIR}/.artifacts/${CT_NAME}-export.tar.gz" -C /var/lib/lxc $CT_NAME/

    echo ""
    read -p "Compression finished. Do you want to DELETE the LXC container from this host? [y/N]: " DELETE_CHOICE
    if [[ "$DELETE_CHOICE" =~ ^[Yy]$ ]]; then
        echo "Deleting container $CT_NAME..."
        lxc-destroy -n $CT_NAME
        echo "Container successfully removed."
    else
        echo ""
        read -p "Do you want to restart the container on this host? [y/N]: " RESTART_CHOICE
        if [[ "$RESTART_CHOICE" =~ ^[Yy]$ ]]; then
            echo "Restarting $CT_NAME..."
            lxc-start -n $CT_NAME -d
            lxc-ls -f
        else
            echo "Leaving $CT_NAME stopped."
        fi
    fi
else
    echo ""
    read -p "Do you want to keep the container running on this host? [Y/n]: " KEEP_RUNNING_CHOICE
    if [[ "$KEEP_RUNNING_CHOICE" =~ ^[Nn]$ ]]; then
        echo "Stopping $CT_NAME..."
        lxc-stop -n $CT_NAME
        lxc-ls -f
    else
        echo "Container $CT_NAME is running."
    fi
fi

echo ""
echo "Script finished!"
