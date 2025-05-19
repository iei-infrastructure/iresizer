fn main() {
  let mut res = winres::WindowsResource::new();
  res.set("ProductName", "iResizer");
  res.set("FileDescription", "A fast and flexible image resizer for a single file or all files in a directory");
  res.set("CompanyName", "PT. Indonesia Epson Industry");
  res.set("LegalCopyright", "© 2025 PT. Indonesia Epson Industry");
  res.set("OriginalFilename", "iResizer.exe");
  res.set("FileVersion", "0.1.1.0");
  res.set("ProductVersion", "0.1.1.0");
  res.compile().expect("Failed to compile resources");
}
