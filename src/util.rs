pub trait MoreCharMethod {
    fn is_end_of_name(&self) -> bool;

    fn is_newline(&self) -> bool;

    fn is_exponent(&self) -> bool;
}

pub trait MoreStrMethod {
    fn skip_whitespace(&self, index: usize) -> Option<usize>;

    fn skip_line(&self, index: usize) -> Option<usize>;
}

impl MoreCharMethod for char {
    #[inline]
    fn is_newline(&self) -> bool {
        match *self {
            '\n' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_exponent(&self) -> bool {
        match *self {
            'e' | 'E' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_end_of_name(&self) -> bool {
        match *self {
            '\t' | '\r' | '\n' => true,
            _ => false,
        }
    }
}

impl MoreStrMethod for Vec<u8> {
    /* 
     * Skips the whitespaces in the text data.
     * And returns a new index of current position after this operation whitin Some(T).
     * Otherwise, returns None.
     */
    #[inline]
    fn skip_whitespace(&self, mut index: usize) -> Option<usize> {
        let len = self.len();
        while <u8 as Into<char>>::into(self[index]).is_whitespace() && index < len {
            index += 1;
        }

        if index == len {
            None
        } else {
            Some(index)
        }
    }

    /*
     * Skips the line in the text data.
     * And returns a new index of current position after this operation within Some(T).
     * Otherwise, returns None.
     */
    #[inline]
    fn skip_line(&self, mut index: usize) -> Option<usize> {
        let len = self.len();
        while <u8 as Into<char>>::into(self[index]).is_newline() && index < len {
            index += 1;
        }

        if index == len {
            None
        } else {
            Some(index)
        }
    }
}
