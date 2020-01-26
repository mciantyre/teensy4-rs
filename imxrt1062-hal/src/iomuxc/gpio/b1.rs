// No alt7...

pad!(
    GPIO_B1_00,
    sw_mux_ctl_pad_gpio_b1_00,
    sw_pad_ctl_pad_gpio_b1_00,
    SW_PAD_CTL_PAD_GPIO_B1_00,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_01,
    sw_mux_ctl_pad_gpio_b1_01,
    sw_pad_ctl_pad_gpio_b1_01,
    SW_PAD_CTL_PAD_GPIO_B1_01,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_02,
    sw_mux_ctl_pad_gpio_b1_02,
    sw_pad_ctl_pad_gpio_b1_02,
    SW_PAD_CTL_PAD_GPIO_B1_02,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_03,
    sw_mux_ctl_pad_gpio_b1_03,
    sw_pad_ctl_pad_gpio_b1_03,
    SW_PAD_CTL_PAD_GPIO_B1_03,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

// ... and now we drop alt6...

pad!(
    GPIO_B1_04,
    sw_mux_ctl_pad_gpio_b1_04,
    sw_pad_ctl_pad_gpio_b1_04,
    SW_PAD_CTL_PAD_GPIO_B1_04,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

pad!(
    GPIO_B1_05,
    sw_mux_ctl_pad_gpio_b1_05,
    sw_pad_ctl_pad_gpio_b1_05,
    SW_PAD_CTL_PAD_GPIO_B1_05,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

pad!(
    GPIO_B1_06,
    sw_mux_ctl_pad_gpio_b1_06,
    sw_pad_ctl_pad_gpio_b1_06,
    SW_PAD_CTL_PAD_GPIO_B1_06,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

pad!(
    GPIO_B1_07,
    sw_mux_ctl_pad_gpio_b1_07,
    sw_pad_ctl_pad_gpio_b1_07,
    SW_PAD_CTL_PAD_GPIO_B1_07,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

// ... and alt6 is back!

pad!(
    GPIO_B1_08,
    sw_mux_ctl_pad_gpio_b1_08,
    sw_pad_ctl_pad_gpio_b1_08,
    SW_PAD_CTL_PAD_GPIO_B1_08,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_09,
    sw_mux_ctl_pad_gpio_b1_09,
    sw_pad_ctl_pad_gpio_b1_09,
    SW_PAD_CTL_PAD_GPIO_B1_09,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

// ... buuuuuuuuut now we drop alt8...

pad!(
    GPIO_B1_10,
    sw_mux_ctl_pad_gpio_b1_10,
    sw_pad_ctl_pad_gpio_b1_10,
    SW_PAD_CTL_PAD_GPIO_B1_10,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt9]
);

pad!(
    GPIO_B1_11,
    sw_mux_ctl_pad_gpio_b1_11,
    sw_pad_ctl_pad_gpio_b1_11,
    SW_PAD_CTL_PAD_GPIO_B1_11,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt9]
);

// SIKE THERE'S NO ALT0 HERE!!!!!!!
//
// The compiler caught this, which is aweseome!
// We don't have to worry about adding incorrect
// alternatives; we only need concern ourselves about
// missing alternatives. Thanks, svd2rust!
pad!(
    GPIO_B1_12,
    sw_mux_ctl_pad_gpio_b1_12,
    sw_pad_ctl_pad_gpio_b1_12,
    SW_PAD_CTL_PAD_GPIO_B1_12,
    [alt1, alt2, alt3, alt4, alt5, alt6, alt9]
);

// ... bring back alt0, and alt8.

pad!(
    GPIO_B1_13,
    sw_mux_ctl_pad_gpio_b1_13,
    sw_pad_ctl_pad_gpio_b1_13,
    SW_PAD_CTL_PAD_GPIO_B1_13,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_14,
    sw_mux_ctl_pad_gpio_b1_14,
    sw_pad_ctl_pad_gpio_b1_14,
    SW_PAD_CTL_PAD_GPIO_B1_14,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_15,
    sw_mux_ctl_pad_gpio_b1_15,
    sw_pad_ctl_pad_gpio_b1_15,
    SW_PAD_CTL_PAD_GPIO_B1_15,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);
