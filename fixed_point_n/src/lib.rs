#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
     fn from (n: f64) -> Self {
         if n >= 1.0 {
             Q7(127)
         } else if n <= -1.0 {
             Q7(-128)
         } else {
             Q7((n * 128.0) as i8)
         }
     }
}

impl From<Q7> for f64 {
     fn from(n: Q7) -> f64 {
         (n.0 as f64) * 2f64.powf(-7.0)
     }
}

impl From<f32> for Q7 {
     fn from (n: f32) -> Self {
         Q7::from(n as f64)
     }
}

impl From<Q7> for f32 {
     fn from(n: Q7) -> f32 {
         f64::from(n) as f32
     }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Q412(i16);

impl From<f64> for Q412{
    fn from(n: f64) -> Self {
         if n >= 8.0 {
             Q412(32767)
         } else if n <= -8.0 {
             Q412(-32768)
         } else {
            // positive n
            let n = n * (1<<12) as f64; // 12 is number of bits for fractional part
            let m = n.round() as i16;
            Q412(m)
         }
    }
}

impl From<Q412> for f64{
    fn from(n: Q412) -> f64 {
        (n.0 as f64)/(1<<12) as f64
    }
}

impl From<Q412> for f32{
    fn from(n: Q412) -> f32 {
        f64::from(n) as f32
    }
}


impl From<f32> for Q412{
    fn from(n: f32) -> Self {
        Q412::from(n as f64)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bonds_q7() {
        assert_eq!(Q7::from(10.01211), Q7::from(1.0));
        assert_eq!(Q7::from(-10.01211), Q7::from(-1.0));
    }
    #[test]
    fn out_of_bonds_q412() {
        assert_eq!(Q412::from(-100.0), Q412::from(-8.0));
        assert_eq!(Q412::from(1100.0), Q412::from(8.0))
    }

    #[test]
    fn from_f32_to_q7() {
        assert_eq!(Q7::from(0.7_f32),Q7(89));
        assert_eq!(Q7::from(-0.378_f32),Q7(-48));
        assert_eq!(Q7::from(0.00_f32),Q7(0));
        assert_eq!(Q7::from(2.00_f32),Q7(127));
        assert_eq!(Q7::from(-3.00_f32),Q7(-128));
    }

    #[test]
    fn from_f32_to_q412() {
        assert_eq!(Q412::from(0.7_f32), Q412(2867));
        assert_eq!(Q412::from(-0.378_f32), Q412(-1548));
        assert_eq!(Q412::from(0.00_f32), Q412(0));
        assert_eq!(Q412::from(2.00_f32), Q412(8192));
        assert_eq!(Q412::from(-3.00_f32), Q412(-12288));
    }


    #[test]
    fn from_f64_to_q7() {
        assert_eq!(Q7::from(0.7_f64),Q7(89));
        assert_eq!(Q7::from(-0.378_f64),Q7(-48));
        assert_eq!(Q7::from(0.00_f64),Q7(0));
        assert_eq!(Q7::from(2.00_f64),Q7(127));
        assert_eq!(Q7::from(-3.00_f64),Q7(-128));
    }

    #[test]
    fn from_f64_to_q412() {
        assert_eq!(Q412::from(3.14_f64), Q412(12861));
        assert_eq!(Q412::from(-3.14_f64), Q412(-12861));
        assert_eq!(Q412::from(0.7_f64),Q412(2867));
        assert_eq!(Q412::from(-0.378_f64),Q412(-1548));
        assert_eq!(Q412::from(0.00_f64),Q412(0));
    }

    #[test]
    fn from_q7_to_f32() {
        let a = Q7(98);
        let b = Q7(-66);
        assert_eq!(a,f32::from(a).into());
        assert_eq!(b,f32::from(b).into());
    }

    #[test]
    fn from_q412_to_f32() {
        let a = Q412(98);
        let b = Q412(-66);
        assert_eq!(a, f32::from(a).into());
        assert_eq!(b, f32::from(b).into());
    }

    #[test]
    fn from_q7_to_f64() {
        let a = Q7(127);
        let b = Q7(-128);
        assert_eq!(a,f64::from(a).into());
        assert_eq!(b,f64::from(b).into());
    }

    #[test]
    fn from_q412_to_f64() {
        let a = Q412(127);
        assert_eq!(a, f64::from(a).into());
        let b = Q412(-128);
        assert_eq!(b,f64::from(b).into());
    }
}
