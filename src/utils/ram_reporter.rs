#[inline(never)]
pub fn report_free_ram() {
    let sp: u32;
    unsafe { core::arch::asm!("mov {}, sp", out(reg) sp) };

    let heap_end = 0x2004_2000u32; // RP2040 end of SRAM
    let free = heap_end - sp;

    defmt::info!("SP={:08x}  free_RAM ≈ {} KiB", sp, free / 1024);
}