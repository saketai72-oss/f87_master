# Bộ Điều Khiển LED RGB Bàn Phím AULA F87 (`f87_master`)

[🇻🇳 Tiếng Việt](README_VI.md) | [🇬🇧 English](README.md)

Phần mềm mã nguồn mở điều khiển LED RGB từng phím (per-key) hiệu năng cao và bộ công cụ tạo animation cho bàn phím cơ **AULA F87 / F87 Pro** (hỗ trợ cả kết nối Dây USB-C `258a:010c` và Không dây 2.4GHz Dongle `3554:fa09`), được phát triển thuần bằng Rust.

---

## ⚡ Tính Năng Nổi Bật

- **Điều Khiển Chuẩn ANSI 87 Phím (TKL)**: Từng phím được điều khiển chính xác theo tọa độ thực tế mà không cần cài đặt phần mềm độc quyền nặng nề hay tài khoản đám mây của hãng.
- **Kết Nối Kép Siêu Mượt (Dây USB-C & Không Dây 2.4G 21-25 FPS)**: Tự động nhận diện và chuyển đổi linh hoạt. Chế độ 2.4GHz Dongle được tối ưu hóa đặc biệt với thuật toán nén màu thích ứng động (Dynamic Adaptive Quantization) và pacing vô tuyến 82 chunks/s, đạt tốc độ **21–25 FPS mượt mà**, dỡ bỏ mọi rào cản animation và equalizer trên không dây.
- **Tự Động Chạy Ngầm Dưới Khay Hệ Thống (System Tray)**: Bấm nút đóng cửa sổ `[X]` sẽ tự động thu nhỏ ứng dụng xuống khay hệ thống, giúp hiệu ứng đèn LED tiếp tục hoạt động liên tục mà không làm vướng màn hình làm việc.
- **Lưu Trạng Thái Đèn Vào Phần Cứng**: Khi bạn thoát ứng dụng hoàn toàn (`Quit`), hệ thống sẽ tự động lưu frame màu hiện tại vào bộ nhớ flash tĩnh của bàn phím (`kb.set_static`), giữ cho bàn phím luôn sáng đẹp thay vì bị tắt tối đen.
- **Hiệu Ứng Nháy Theo Nhạc (Music Equalizer)**: Bộ lọc đồ họa 16 dải tần số âm thanh (16-band graphic equalizer) chân thực với độ nảy của dải bass, sóng xung kích (beat pulse) trên phím Space và hiển thị đỉnh dB (peak hold).
- **Thư Viện Hiệu Ứng Phong Phú**:
  - `Music Equalizer`: Hiển thị dải tần âm thanh nháy theo nhạc sống động.
  - `Cyber Pulse`: Sóng synthwave 2 tông màu Cyan / Magenta đậm chất cyberpunk.
  - `Matrix Rain`: Mưa mã số kỹ thuật số đổ dọc các hàng phím.
  - `Fire`: Tàn đóm lửa bốc lên ngẫu nhiên với độ lập lòe tự nhiên.
  - `Aurora`: Dải lụa cực quang lấp lánh sắc xanh ngọc và tím pastel.
  - `Wave`, `Breathe`, `Ripple`, `Spiral`, `Bloom`, `Sweep`, `Solid`, `Text`.
- **Nút "Set Default" Riêng Biệt**: Mỗi thanh trượt tốc độ, độ sáng và bảng chọn màu đều có nút bấm đặt lại giá trị mặc định riêng tiện lợi.
- **Giao Diện Tinh Gọn & Hiện Đại**: Loại bỏ hoàn toàn thanh sidebar cồng kềnh; mô phỏng bàn phím trực tiếp (live visualizer); chuyển đổi thiết bị nhanh ở thanh công cụ phía trên.
- **Trình Soạn Thảo Hoạt Ảnh & Keyframe**: Vẽ màu theo trục thời gian (timeline), phân tầng layer mặt nạ (masks), hỗ trợ chèn file GIF và hình ảnh.
- **Bộ Cài Đặt Độc Lập**: Tích hợp bộ cài đặt Windows native (`.exe`) tự động tạo shortcut Desktop, Start Menu và kèm trình gỡ cài đặt sạch sẽ.

---

## 📦 Tải Về & Cài Đặt

Các bản phát hành sẵn sàng để sử dụng tại thư mục [`release_package/`](release_package/) hoặc trang [GitHub Releases](https://github.com/saketai72-oss/f87_master/releases):

| Tệp tin | Mô tả |
| --- | --- |
| **[`AULA-F87-Setup-x64.exe`](release_package/AULA-F87-Setup-x64.exe)** | Bộ cài đặt Windows hoàn chỉnh (Menu Start, Shortcut Desktop, Trình gỡ cài đặt) |
| **[`f87.exe`](release_package/f87.exe)** | Bản Portable chạy ngay không cần cài đặt |

---

## 💡 Lưu Ý Về Phần Cứng AULA F87

Trên phần cứng bàn phím AULA F87, **2 dải LED viền bên hông** và **thanh LED báo pin ở giữa** được điều khiển độc lập bằng một chip phụ (MCU sub-controller) riêng biệt của bo mạch phần cứng. Bạn có thể chỉnh chúng trực tiếp bằng các tổ hợp phím tắt có sẵn trên bàn phím:

- **`Fn + Shift Phải`**: Đổi chế độ LED viền (Sóng, Thở, Tĩnh, Neon, Tắt).
- **`Fn + Ctrl Phải`**: Đổi màu dải LED viền.
- **`Fn + Alt Phải`**: Chỉnh độ sáng LED viền.
- **`Fn + >`**: Chỉnh tốc độ hiệu ứng LED viền.
- **`Fn + B`**: Hiển thị phần trăm dung lượng pin còn lại trên thanh LED giữa.

Phần mềm điều khiển luồng dữ liệu 87 phím cơ trung tâm với độ chính xác tuyệt đối và không gây trễ tín hiệu gõ phím.

---

## 🛠️ Biên Dịch Từ Mã Nguồn (Build from Source)

### Yêu cầu cài đặt
- **Rust Toolchain**: 1.75+ (`stable-x86_64-pc-windows-msvc` hoặc `gnu`)
- **Git**

```powershell
# Sao chép kho lưu trữ về máy
git clone https://github.com/saketai72-oss/f87_master.git
cd f87_master

# Chạy toàn bộ các bài kiểm thử tự động
cargo test --workspace

# Biên dịch ứng dụng bản Release
cargo build --release -p aula-app --bin f87

# Biên dịch bộ cài đặt Windows độc lập
cargo build --release --manifest-path packaging/windows/installer/Cargo.toml
```

Đường dẫn tệp tin sau khi biên dịch:
- Ứng dụng: `target/release/f87.exe`
- Bộ cài đặt: `packaging/windows/installer/target/release/AULA-F87-Setup-x64.exe`

---

## 🏗️ Cấu Trúc Dự Án

```
f87_master/
├── crates/
│   ├── aula-protocol/   # Giao thức HID SinoWealth & Compx, đóng gói dữ liệu và định thời vô tuyến
│   ├── aula-effects/    # Trình thực thi Rhai script, engine keyframe timeline và quản lý layer
│   └── aula-app/        # Giao diện đồ họa Egui/Eframe, render thread và cài đặt người dùng
├── effects/             # Các tệp script hiệu ứng .rhai (tự động tải lại khi chỉnh sửa)
├── packaging/windows/   # Trình tạo bộ cài đặt Windows độc lập viết bằng Rust
└── release_package/     # Các tệp thực thi đã đóng gói sẵn sàng phân phối
```

---

## 📜 Giấy Phép (License)

Dự án được phân phối dưới giấy phép kép [MIT](LICENSE-MIT) hoặc [Apache-2.0](LICENSE-APACHE).
