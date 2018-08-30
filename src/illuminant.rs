//! Spectral data of reference illuminants

use super::spectral_power_distribution::SPD;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref D65: SPD = {
        SPD::from_wavelength_and_value(&[
            300.0, 305.0, 310.0, 315.0, 320.0, 325.0, 330.0, 335.0, 340.0,
            345.0, 350.0, 355.0, 360.0, 365.0, 370.0, 375.0, 380.0, 385.0,
            390.0, 395.0, 400.0, 405.0, 410.0, 415.0, 420.0, 425.0, 430.0,
            435.0, 440.0, 445.0, 450.0, 455.0, 460.0, 465.0, 470.0, 475.0,
            480.0, 485.0, 490.0, 495.0, 500.0, 505.0, 510.0, 515.0, 520.0,
            525.0, 530.0, 535.0, 540.0, 545.0, 550.0, 555.0, 560.0, 565.0,
            570.0, 575.0, 580.0, 585.0, 590.0, 595.0, 600.0, 605.0, 610.0,
            615.0, 620.0, 625.0, 630.0, 635.0, 640.0, 645.0, 650.0, 655.0,
            660.0, 665.0, 670.0, 675.0, 680.0, 685.0, 690.0, 695.0, 700.0,
            705.0, 710.0, 715.0, 720.0, 725.0, 730.0, 735.0, 740.0, 745.0,
            750.0, 755.0, 760.0, 765.0, 770.0, 775.0, 780.0,
        ],
        &[
            0.034100, 1.664300, 3.294500, 11.765200, 20.236000, 28.644700,
            37.053500, 38.501100, 39.948800, 42.430200, 44.911700, 45.775000,
            46.638300, 49.363700, 52.089100, 51.032300, 49.975500, 52.311800,
            54.648200, 68.701500, 82.754900, 87.120400, 91.486000, 92.458900,
            93.431800, 90.057000, 86.682300, 95.773600, 104.865000, 110.936000,
            117.008000, 117.410000, 117.812000, 116.336000, 114.861000,
            115.392000, 115.923000, 112.367000, 108.811000, 109.082000,
            109.354000, 108.578000, 107.802000, 106.296000, 104.790000,
            106.239000, 107.689000, 106.047000, 104.405000, 104.225000,
            104.046000, 102.023000, 100.000000, 98.167100, 96.334200,
            96.061100, 95.788000, 92.236800, 88.685600, 89.345900, 90.006200,
            89.802600, 89.599100, 88.648900, 87.698700, 85.493600, 83.288600,
            83.493900, 83.699200, 81.863000, 80.026800, 80.120700, 80.214600,
            81.246200, 82.277800, 80.281000, 78.284200, 74.002700, 69.721300,
            70.665200, 71.609100, 72.979000, 74.349000, 67.976500, 61.604000,
            65.744800, 69.885600, 72.486300, 75.087000, 69.339800, 63.592700,
            55.005400, 46.418200, 56.611800, 66.805400, 65.094100, 63.382800,
        ])
    };

    pub static ref Ones: SPD = {
        SPD::from_wavelength_and_value(&[
            300.0, 310.0, 320.0, 330.0, 340.0, 350.0, 360.0, 370.0, 380.0, 390.0,
            400.0, 410.0, 420.0, 430.0, 440.0, 450.0, 460.0, 470.0, 480.0, 490.0,
            500.0, 510.0, 520.0, 530.0, 540.0, 550.0, 560.0, 570.0, 580.0, 590.0,
            600.0, 610.0, 620.0, 630.0, 640.0, 650.0, 660.0, 670.0, 680.0, 690.0,
            700.0, 710.0, 720.0, 730.0, 740.0, 750.0, 760.0, 770.0, 780.0, 790.0,
        ], 
        &[
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ])
    };
}
