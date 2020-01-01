void call(void)
{
    asm("dsb");
    asm("isb");
}