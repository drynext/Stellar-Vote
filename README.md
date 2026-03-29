# StellarVote: Decentralized Voting Platform

**StellarVote** là một ứng dụng bỏ phiếu phi tập trung (dApp) được xây dựng trên mạng lưới **Stellar**, sử dụng hợp đồng thông minh **Soroban** để đảm bảo tính minh bạch, bảo mật và không thể gian lận trong các cuộc bầu cử nội bộ.

---

## Thông tin dự án

* **Tên dự án:** StellarVote
* **Vấn đề:** Các cuộc bầu cử tại câu lạc bộ, trường học hoặc khu dân cư hiện nay thường thiếu minh bạch, dễ bị can thiệp kết quả và tốn thời gian kiểm phiếu thủ công.
* **Giải pháp:** Sử dụng Blockchain để biến mỗi phiếu bầu thành một giao dịch duy nhất, không thể thay đổi và có thể kiểm chứng công khai bởi bất kỳ ai.

---

## Tính năng Stellar sử dụng

* **Soroban Smart Contracts:** Trái tim của dự án, quản lý logic bỏ phiếu, kiểm tra quyền hạn cử tri và lưu trữ kết quả tự động.
* **Custom Tokens (VOTE):** Mỗi cử tri hợp lệ sẽ được cấp một lượng token "VOTE" định danh để thực hiện quyền biểu quyết.
* **Trustlines:** Đảm bảo chỉ những người dùng được hệ thống xác thực mới có thể nhận và sử dụng token bỏ phiếu.

---

## Người dùng mục tiêu
* **Sinh viên:** Bầu ban cán sự lớp, ban điều hành câu lạc bộ.
* **Cư dân:** Biểu quyết các vấn đề trong khu chung cư hoặc tổ dân phố.
* **Tổ chức phi lợi nhuận:** Quyết định phương án phân bổ quỹ từ thiện.

---

## Tính năng cốt lõi (MVP)
**Giao dịch cốt lõi:** Một lời gọi hàm (Function Call) đến Smart Contract:
`invoke vote(voter_address, proposal_id)`
Giao dịch này sẽ:
1. Xác thực ví người dùng có quyền bầu hay không.
2. Kiểm tra xem người dùng đã bầu chưa (tránh Double Voting).
3. Cộng điểm cho đề xuất và ghi lại mã định danh giao dịch (Transaction ID) để đối soát.

---

## Tại sao chọn Stellar?

| Tiêu chí | Hệ thống truyền thống | Stellar Network |
| :--- | :--- | :--- |
| **Chi phí** | Tốn kém (In ấn, nhân sự) | **~0.0001 USD** mỗi phiếu |
| **Thời gian** | Nhiều giờ/ngày để kiểm phiếu | **~5 giây** (Tức thì) |
| **Tính minh bạch** | Dễ bị nghi ngờ gian lận | **Bất biến** (Công khai trên Ledger) |
| **Khả năng tiếp cận** | Cần có mặt trực tiếp | **Mọi nơi** qua ví điện tử |

---
