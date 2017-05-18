use Internal;

impl<T> Internal<T> for bool {
    type Array = [T; 2];
    fn slice(array: &[T; 2]) -> &[T] {
        array
    }
    fn slice_mut(array: &mut [T; 2]) -> &mut [T] {
        array
    }
    fn from_usize(value: usize) -> Self {
        match value {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
    fn to_usize(self) -> usize {
        self as usize
    }
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> [T; 2] {
        [f(false), f(true)]
    }
}

impl<T> Internal<T> for u8 {
    type Array = [T; 256];
    fn slice(array: &[T; 256]) -> &[T] {
        array
    }
    fn slice_mut(array: &mut [T; 256]) -> &mut [T] {
        array
    }
    fn from_usize(value: usize) -> Self {
        value as u8
    }
    fn to_usize(self) -> usize {
        self as usize
    }
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> [T; 256] {
        // This was initially a macro, but let's say that compiling for
        // a minute or so was a little bit too much
        // (f(0 + 1 + 1 + 1 + ...) seemed like a good idea at the time)
        [f(0), f(1), f(2), f(3), f(4), f(5), f(6), f(7), f(8), f(9), f(10), f(11), f(12), f(13),
         f(14), f(15), f(16), f(17), f(18), f(19), f(20), f(21), f(22), f(23), f(24), f(25), f(26),
         f(27), f(28), f(29), f(30), f(31), f(32), f(33), f(34), f(35), f(36), f(37), f(38), f(39),
         f(40), f(41), f(42), f(43), f(44), f(45), f(46), f(47), f(48), f(49), f(50), f(51), f(52),
         f(53), f(54), f(55), f(56), f(57), f(58), f(59), f(60), f(61), f(62), f(63), f(64), f(65),
         f(66), f(67), f(68), f(69), f(70), f(71), f(72), f(73), f(74), f(75), f(76), f(77), f(78),
         f(79), f(80), f(81), f(82), f(83), f(84), f(85), f(86), f(87), f(88), f(89), f(90), f(91),
         f(92), f(93), f(94), f(95), f(96), f(97), f(98), f(99), f(100), f(101), f(102), f(103),
         f(104), f(105), f(106), f(107), f(108), f(109), f(110), f(111), f(112), f(113), f(114),
         f(115), f(116), f(117), f(118), f(119), f(120), f(121), f(122), f(123), f(124), f(125),
         f(126), f(127), f(128), f(129), f(130), f(131), f(132), f(133), f(134), f(135), f(136),
         f(137), f(138), f(139), f(140), f(141), f(142), f(143), f(144), f(145), f(146), f(147),
         f(148), f(149), f(150), f(151), f(152), f(153), f(154), f(155), f(156), f(157), f(158),
         f(159), f(160), f(161), f(162), f(163), f(164), f(165), f(166), f(167), f(168), f(169),
         f(170), f(171), f(172), f(173), f(174), f(175), f(176), f(177), f(178), f(179), f(180),
         f(181), f(182), f(183), f(184), f(185), f(186), f(187), f(188), f(189), f(190), f(191),
         f(192), f(193), f(194), f(195), f(196), f(197), f(198), f(199), f(200), f(201), f(202),
         f(203), f(204), f(205), f(206), f(207), f(208), f(209), f(210), f(211), f(212), f(213),
         f(214), f(215), f(216), f(217), f(218), f(219), f(220), f(221), f(222), f(223), f(224),
         f(225), f(226), f(227), f(228), f(229), f(230), f(231), f(232), f(233), f(234), f(235),
         f(236), f(237), f(238), f(239), f(240), f(241), f(242), f(243), f(244), f(245), f(246),
         f(247), f(248), f(249), f(250), f(251), f(252), f(253), f(254), f(255)]
    }
}
