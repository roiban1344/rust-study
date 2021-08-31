const TABLE: [(i64, i64); 64] = [
    (1, 0xd76aa478),
    (2, 0xe8c7b756),
    (3, 0x242070db),
    (4, 0xc1bdceee),
    (5, 0xf57c0faf),
    (6, 0x4787c62a),
    (7, 0xa8304613),
    (8, 0xfd469501),
    (9, 0x698098d8),
    (10, 0x8b44f7af),
    (11, 0xffff5bb1),
    (12, 0x895cd7be),
    (13, 0x6b901122),
    (14, 0xfd987193),
    (15, 0xa679438e),
    (16, 0x49b40821),
    (17, 0xf61e2562),
    (18, 0xc040b340),
    (19, 0x265e5a51),
    (20, 0xe9b6c7aa),
    (21, 0xd62f105d),
    (22, 0x2441453),
    (23, 0xd8a1e681),
    (24, 0xe7d3fbc8),
    (25, 0x21e1cde6),
    (26, 0xc33707d6),
    (27, 0xf4d50d87),
    (28, 0x455a14ed),
    (29, 0xa9e3e905),
    (30, 0xfcefa3f8),
    (31, 0x676f02d9),
    (32, 0x8d2a4c8a),
    (33, 0xfffa3942),
    (34, 0x8771f681),
    (35, 0x6d9d6122),
    (36, 0xfde5380c),
    (37, 0xa4beea44),
    (38, 0x4bdecfa9),
    (39, 0xf6bb4b60),
    (40, 0xbebfbc70),
    (41, 0x289b7ec6),
    (42, 0xeaa127fa),
    (43, 0xd4ef3085),
    (44, 0x4881d05),
    (45, 0xd9d4d039),
    (46, 0xe6db99e5),
    (47, 0x1fa27cf8),
    (48, 0xc4ac5665),
    (49, 0xf4292244),
    (50, 0x432aff97),
    (51, 0xab9423a7),
    (52, 0xfc93a039),
    (53, 0x655b59c3),
    (54, 0x8f0ccc92),
    (55, 0xffeff47d),
    (56, 0x85845dd1),
    (57, 0x6fa87e4f),
    (58, 0xfe2ce6e0),
    (59, 0xa3014314),
    (60, 0x4e0811a1),
    (61, 0xf7537e82),
    (62, 0xbd3af235),
    (63, 0x2ad7d2bb),
    (64, 0xeb86d391),
];

fn main() {
    for (i, t) in TABLE {
        let t_f32 = t_f32(i);
        let t_f64 = t_f64(i);
        println!("{:>2}\t{:>10}\t{:>5}\t{}", i, t, t_f32 - t, t_f64 - t);
    }
}

fn t_f32(i: i64) -> i64 {
    let sin = (i as f32).sin();
    let x = sin.abs() * (4294967296_i64 as f32);
    x.floor() as i64
}

fn t_f64(i: i64) -> i64 {
    let sin = (i as f64).sin();
    let x = sin.abs() * (4294967296_i64 as f64);
    x.floor() as i64
}
