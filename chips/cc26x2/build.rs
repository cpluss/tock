extern crate cc;

fn main() {
    cc::Build::new()
        .files(vec![
            "src/lib/cc26x2/driverlib/setup.c",
            "src/lib/cc26x2/driverlib/setup_rom.c",
            "src/lib/cc26x2/driverlib/aux_sysif.c",
            "src/lib/cc26x2/driverlib/chipinfo.c",
            "src/lib/cc26x2/driverlib/osc.c",
            "src/lib/cc26x2/driverlib/sys_ctrl.c",
            "src/lib/cc26x2/driverlib/ddi.c",
            "src/lib/cc26x2/driverlib/ioc.c",
        ])
        .include("src/lib/cc26x2/inc")
        .include("src/lib/cc26x2/driverlib")
        //.define("DRIVERLIB_NOROM", None)
        .target("thumbv7em-none-eabi")
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .compile("cc26x2ware");
}