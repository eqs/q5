//
// This utility module build on Kitao's Pyxel code:
// https://github.com/kitao/pyxel
//
// ---
//
// MIT License
//
// Copyright (c) 2018 Takashi Kitao
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

macro_rules! type_error {
    ($msg: expr) => {
        return Err(pyo3::exceptions::PyTypeError::new_err($msg))
    };
}

macro_rules! type_switch {
    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block) => {
        if let Ok($var) = <$type1>::extract($var) {
            $block1
        } else if let Ok($var) = <$type2>::extract($var) {
            $block2
        } else {
            type_error!(format!(
                "must be {} or {}",
                stringify!($type1),
                stringify!($type2)
            ));
        }
    };

    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block, $type3: ty, $block3: block) => {
        if let Ok($var) = <$type1>::extract($var) {
            $block1
        } else if let Ok($var) = <$type2>::extract($var) {
            $block2
        } else if let Ok($var) = <$type3>::extract($var) {
            $block3
        } else {
            type_error!(format!(
                "must be {}, {}, or {}",
                stringify!($type1),
                stringify!($type2),
                stringify!($type3),
            ));
        }
    };

    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block, $type3: ty, $block3: block, $type4: ty, $block4: block) => {
        if let Ok($var) = <$type1>::extract($var) {
            $block1
        } else if let Ok($var) = <$type2>::extract($var) {
            $block2
        } else if let Ok($var) = <$type3>::extract($var) {
            $block3
        } else if let Ok($var) = <$type4>::extract($var) {
            $block4
        } else {
            type_error!(format!(
                "must be {}, {}, {}, or {}",
                stringify!($type1),
                stringify!($type2),
                stringify!($type3),
                stringify!($type4)
            ));
        }
    };
}
