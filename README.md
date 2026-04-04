## Rust Clean Architecture with Nuxt Framework
menggunakan podman untuk containerization, dan quadlet untuk mengelola container, pod, volume, dan jaringan pakai unit systemd

### build images
```bash
podman build -t aeonboarding-backend ./backend
podman build -t localhost/aeonboarding-backend:latest -f backend/Containerfile ./backend
podman build --no-cache --progress plain -t localhost/aeonboarding-backend:latest -f backend/Containerfile ./backend
podman build -t aeonboarding-frontend ./frontend
podman build -t localhost/aeonboarding-frontend:latest -f frontend/Containerfile ./frontend
podman build --no-cache --progress plain -t localhost/aeonboarding-frontend:latest -f frontend/Containerfile ./frontend

```
### konfigurasi quadlet
```bash
mkdir -p ~/.config/systemd/user/
cp quadlets/* ~/.config/systemd/user/

# link container ke systemd
mkdir -p ~/.config/containers/systemd/aeonboarding-nuxt
ln -sf ~/programming/rust/cargoRs/nuxt-podman/quadlets/* ~/.config/containers/systemd/aeonboarding-nuxt/
ln -sf ~/programming/rust/cargoRs/nuxt-podman/quadlets/*.container ~/.config/containers/systemd/aeonboarding-nuxt/
ln -sf ~/programming/rust/cargoRs/nuxt-podman/quadlets/aeonboarding-net.network ~/.config/containers/systemd/aeonboarding-nuxt/aeonboarding.network

systemctl --user daemon-reload
```
### jalankan network dan service yang lain
```bash
systemctl --user start aeonboarding-network.service

# Jalankan sisanya
systemctl --user start aeonboarding-db.service
systemctl --user start aeonboarding-backend.service
systemctl --user start aeonboarding-frontend.service
```

### jalankan container podman
```bash
systemctl --user daemon-reload
systemctl --user start aeonboarding-network.service
systemctl --user start aeonboarding-db.service
```

### masuk ke frontend bash
```bash
podman exec -it aeonboarding-frontend bash
podman run --rm -v .:/app -w /app oven/bun:latest bun install
```


### cek unit list
```bash
systemctl --user list-unit-files | grep aeonboarding
```

### untuk melihat log frontend:
```bash
podman logs -f aeonboarding-frontend-dev
```

### cek container status
```bash
systemctl --user status aeonboarding-network.service
systemctl --user status aeonboarding-db.service
systemctl --user status aeonboarding-backend.service
systemctl --user status aeonboarding-frontend.service
```
### restart container service
```bash
systemctl --user restart aeonboarding-network.service
systemctl --user restart aeonboarding-db.service
systemctl --user restart aeonboarding-backend.service
systemctl --user restart aeonboarding-frontend.service
```

### untuk mencari service tersedia
```bash
systemctl --user list-units --type=service
systemctl --user list-units --type=service | grep aeonboarding
```
### untuk menjalankan migration pada posgresql
```bash
sqlx migrate add -r create_users_table
sqlx database create

sqlx migrate run
```

### bersihkan package
```bash
rm -rf .nuxt .output node_modules/.cache
```
