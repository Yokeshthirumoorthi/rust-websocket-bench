// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// CREDITS
// Project: https://github.com/actix/examples/tree/master/diesel
// Copyright (c) 2017 Nikolay Kim (fafhrd91@gmail.com)
// License (MIT) https://github.com/actix/examples/blob/master/LICENSE

table! {
    users (id) {
        id -> Text,
        name -> Text,
    }
}
