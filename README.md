## Rust Clean Architecture with Nuxt Framework

menggunakan podman untuk containerization, dan quadlet untuk mengelola container, pod, volume, dan jaringan pakai unit systemd

### jalankan seeder 
```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
cargo sqlx prepare
cargo run --bin seed
```

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
# 1. Pastikan nama file di folder project sudah benar (Tanpa Typo)
mv ~/programming/rust/cargoRs/aeonboaring/quadlets/aeonboarding-backend.contaniner ~/programming/rust/cargoRs/aeonboaring/quadlets/aeonboarding-backend.container 2>/dev/null || true

# 2. Buat direktori target Quadlet
mkdir -p ~/.config/containers/systemd/aeonboarding-nuxt

# 3. Hubungkan file project ke sistem Quadlet (Gunakan path absolut)
ln -sf ~/programming/rust/cargoRs/aeonboaring/quadlets/* ~/.config/containers/systemd/aeonboarding-nuxt/

# 4. Beritahu systemd untuk men-generate unit dari file tersebut
systemctl --user daemon-reload

# 5. Cek hasilnya
systemctl --user list-unit-files | grep aeonboarding
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

### berhentikan semua service container
```bash
podman stop --all
```

### bersihkan service database
```bash
systemctl --user stop aeonboarding-db.service
podman rm -f aeonboarding-db
podman volume rm aeonboarding_db_data
```