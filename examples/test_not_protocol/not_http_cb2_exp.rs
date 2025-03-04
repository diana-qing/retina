Filter: !http, Datatypes: ["HttpTransaction"], Callback: "not_http_cb2"
Expecting 1 subsctription(s)
filter_raw: !http
pairs: Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "!http",
                                start: 0,
                                end: 5,
                            },
                            inner: [
                                Pair {
                                    rule: not_op,
                                    span: Span {
                                        str: "!",
                                        start: 0,
                                        end: 1,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "http",
                                        start: 1,
                                        end: 5,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: EOI,
            span: Span {
                str: "",
                start: 5,
                end: 5,
            },
            inner: [],
        },
    ],
)
pair: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "!http",
                    start: 0,
                    end: 5,
                },
                inner: [
                    Pair {
                        rule: not_op,
                        span: Span {
                            str: "!",
                            start: 0,
                            end: 1,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "http",
                            start: 1,
                            end: 5,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: not_op,
                    span: Span {
                        str: "!",
                        start: 0,
                        end: 1,
                    },
                    inner: [],
                },
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "http",
                        start: 1,
                        end: 5,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: not_op,
                span: Span {
                    str: "!",
                    start: 0,
                    end: 1,
                },
                inner: [],
            },
            Pair {
                rule: protocol,
                span: Span {
                    str: "http",
                    start: 1,
                    end: 5,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: not_op,
            span: Span {
                str: "!",
                start: 0,
                end: 1,
            },
            inner: [],
        },
        Pair {
            rule: protocol,
            span: Span {
                str: "http",
                start: 1,
                end: 5,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: not_op,
        span: Span {
            str: "!",
            start: 0,
            end: 1,
        },
        inner: [],
    },
    Pair {
        rule: protocol,
        span: Span {
            str: "http",
            start: 1,
            end: 5,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
            ),
        ],
    ),
]
ast: Disjunct(
    [
        Conjunct(
            [
                Predicate(
                    Unary {
                        protocol: ProtocolName(
                            "http",
                        ),
                    },
                ),
            ],
        ),
    ],
)
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ],
]
ptree: FlatPTree {
    root: FlatPNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        is_terminal: false,
        terminates: None,
        patterns: [
            0,
            1,
        ],
        children: [
            FlatPNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    0,
                ],
                children: [
                    FlatPNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            0,
                        ],
                        children: [
                            FlatPNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    0,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
            FlatPNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    1,
                ],
                children: [
                    FlatPNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            1,
                        ],
                        children: [
                            FlatPNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    1,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    size: 7,
}
filter_subtree: filter: Filter {
    patterns: [
        LayeredPattern(
            {
                ProtocolName(
                    "ipv4",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
        LayeredPattern(
            {
                ProtocolName(
                    "ipv6",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
    ],
}
filter_subtree: patterns: [
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv4",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv6",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
]
filter_subtree: deliver: Deliver {
    id: 0,
    as_str: "not_http_cb2(HttpTransaction)",
    must_deliver: false,
}
filter_subtree: ptree before collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [
                    PNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[PacketContinue],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            0,
                        ],
                        children: [],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
            PNode {
                id: 3,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    1,
                ],
                children: [
                    PNode {
                        id: 4,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[PacketContinue],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            1,
                        ],
                        children: [],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
        ],
        if_else: false,
    },
    size: 5,
    actions: Actions {
        data: ActionData[PacketContinue],
        terminal_actions: ActionData[],
    },
    filter_layer: PacketContinue,
    collapsed: false,
}
filter_subtree: ptree after collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [
                    PNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[PacketContinue],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            0,
                        ],
                        children: [],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
            PNode {
                id: 3,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    1,
                ],
                children: [
                    PNode {
                        id: 4,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[PacketContinue],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            1,
                        ],
                        children: [],
                        if_else: false,
                    },
                ],
                if_else: true,
            },
        ],
        if_else: false,
    },
    size: 5,
    actions: Actions {
        data: ActionData[PacketContinue],
        terminal_actions: ActionData[],
    },
    filter_layer: PacketContinue,
    collapsed: false,
}
update_body: body: [
    TokenStream [
        Ident {
            ident: "result",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '.',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "push",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Punct {
                    ch: '&',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "Actions",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [
                        Ident {
                            ident: "data",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "1",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ',',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "terminal_actions",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "0",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ';',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
    ],
]
update_body: body: [
    TokenStream [
        Ident {
            ident: "if",
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "let",
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "Ok",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Ident {
                    ident: "tcp",
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '=',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '&',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "retina_core",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "protocols",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "packet",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "Packet",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "parse_to",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '<',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "retina_core",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "protocols",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "packet",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "tcp",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "Tcp",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '>',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Ident {
                    ident: "ipv4",
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Brace,
            stream: TokenStream [
                Ident {
                    ident: "result",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: '.',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "push",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [
                        Punct {
                            ch: '&',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "Actions",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Brace,
                            stream: TokenStream [
                                Ident {
                                    ident: "data",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "ActionData",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Joint,
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "from",
                                    span: #56 bytes(1698..1715),
                                },
                                Group {
                                    delimiter: Parenthesis,
                                    stream: TokenStream [
                                        Literal {
                                            kind: Integer,
                                            symbol: "1",
                                            suffix: None,
                                            span: #56 bytes(1698..1715),
                                        },
                                    ],
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ',',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "terminal_actions",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "ActionData",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Joint,
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "from",
                                    span: #56 bytes(1698..1715),
                                },
                                Group {
                                    delimiter: Parenthesis,
                                    stream: TokenStream [
                                        Literal {
                                            kind: Integer,
                                            symbol: "0",
                                            suffix: None,
                                            span: #56 bytes(1698..1715),
                                        },
                                    ],
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ';',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
    ],
]
update_body: body: [
    TokenStream [
        Ident {
            ident: "result",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '.',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "push",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Punct {
                    ch: '&',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "Actions",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [
                        Ident {
                            ident: "data",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "1",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ',',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "terminal_actions",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "0",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ';',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
    ],
]
update_body: body: [
    TokenStream [
        Ident {
            ident: "if",
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "let",
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "Ok",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Ident {
                    ident: "tcp",
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '=',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '&',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "retina_core",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "protocols",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "packet",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "Packet",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "parse_to",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '<',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "retina_core",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "protocols",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "packet",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "tcp",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Joint,
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "Tcp",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '>',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Ident {
                    ident: "ipv6",
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Brace,
            stream: TokenStream [
                Ident {
                    ident: "result",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: '.',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "push",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [
                        Punct {
                            ch: '&',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "Actions",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Brace,
                            stream: TokenStream [
                                Ident {
                                    ident: "data",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "ActionData",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Joint,
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "from",
                                    span: #56 bytes(1698..1715),
                                },
                                Group {
                                    delimiter: Parenthesis,
                                    stream: TokenStream [
                                        Literal {
                                            kind: Integer,
                                            symbol: "1",
                                            suffix: None,
                                            span: #56 bytes(1698..1715),
                                        },
                                    ],
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ',',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "terminal_actions",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "ActionData",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Joint,
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "from",
                                    span: #56 bytes(1698..1715),
                                },
                                Group {
                                    delimiter: Parenthesis,
                                    stream: TokenStream [
                                        Literal {
                                            kind: Integer,
                                            symbol: "0",
                                            suffix: None,
                                            span: #56 bytes(1698..1715),
                                        },
                                    ],
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ';',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
    ],
]
filter_raw: !http
pairs: Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "!http",
                                start: 0,
                                end: 5,
                            },
                            inner: [
                                Pair {
                                    rule: not_op,
                                    span: Span {
                                        str: "!",
                                        start: 0,
                                        end: 1,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "http",
                                        start: 1,
                                        end: 5,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: EOI,
            span: Span {
                str: "",
                start: 5,
                end: 5,
            },
            inner: [],
        },
    ],
)
pair: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "!http",
                    start: 0,
                    end: 5,
                },
                inner: [
                    Pair {
                        rule: not_op,
                        span: Span {
                            str: "!",
                            start: 0,
                            end: 1,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "http",
                            start: 1,
                            end: 5,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: not_op,
                    span: Span {
                        str: "!",
                        start: 0,
                        end: 1,
                    },
                    inner: [],
                },
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "http",
                        start: 1,
                        end: 5,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: not_op,
                span: Span {
                    str: "!",
                    start: 0,
                    end: 1,
                },
                inner: [],
            },
            Pair {
                rule: protocol,
                span: Span {
                    str: "http",
                    start: 1,
                    end: 5,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: not_op,
            span: Span {
                str: "!",
                start: 0,
                end: 1,
            },
            inner: [],
        },
        Pair {
            rule: protocol,
            span: Span {
                str: "http",
                start: 1,
                end: 5,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: not_op,
        span: Span {
            str: "!",
            start: 0,
            end: 1,
        },
        inner: [],
    },
    Pair {
        rule: protocol,
        span: Span {
            str: "http",
            start: 1,
            end: 5,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
            ),
        ],
    ),
]
ast: Disjunct(
    [
        Conjunct(
            [
                Predicate(
                    Unary {
                        protocol: ProtocolName(
                            "http",
                        ),
                    },
                ),
            ],
        ),
    ],
)
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ],
]
ptree: FlatPTree {
    root: FlatPNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        is_terminal: false,
        terminates: None,
        patterns: [
            0,
            1,
        ],
        children: [
            FlatPNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    0,
                ],
                children: [
                    FlatPNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            0,
                        ],
                        children: [
                            FlatPNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    0,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
            FlatPNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    1,
                ],
                children: [
                    FlatPNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            1,
                        ],
                        children: [
                            FlatPNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    1,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    size: 7,
}
filter_subtree: filter: Filter {
    patterns: [
        LayeredPattern(
            {
                ProtocolName(
                    "ipv4",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
        LayeredPattern(
            {
                ProtocolName(
                    "ipv6",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
    ],
}
filter_subtree: patterns: [
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv4",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv6",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
]
filter_subtree: deliver: Deliver {
    id: 0,
    as_str: "not_http_cb2(HttpTransaction)",
    must_deliver: false,
}
filter_subtree: ptree before collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [
                    PNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[ProtoFilter],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            0,
                        ],
                        children: [],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
            PNode {
                id: 3,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    1,
                ],
                children: [
                    PNode {
                        id: 4,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[ProtoFilter],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            1,
                        ],
                        children: [],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
        ],
        if_else: false,
    },
    size: 5,
    actions: Actions {
        data: ActionData[ProtoFilter],
        terminal_actions: ActionData[],
    },
    filter_layer: Packet,
    collapsed: false,
}
filter_subtree: ptree after collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                actions: Actions {
                    data: ActionData[ProtoFilter],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [],
                if_else: false,
            },
            PNode {
                id: 2,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                actions: Actions {
                    data: ActionData[ProtoFilter],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    1,
                ],
                children: [],
                if_else: true,
            },
        ],
        if_else: false,
    },
    size: 3,
    actions: Actions {
        data: ActionData[ProtoFilter],
        terminal_actions: ActionData[],
    },
    filter_layer: Packet,
    collapsed: false,
}
update_body: body: [
    TokenStream [
        Ident {
            ident: "result",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '.',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "push",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Punct {
                    ch: '&',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "Actions",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [
                        Ident {
                            ident: "data",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "32",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ',',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "terminal_actions",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "0",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ';',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
    ],
]
update_body: body: [
    TokenStream [
        Ident {
            ident: "result",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '.',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "push",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Punct {
                    ch: '&',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "Actions",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [
                        Ident {
                            ident: "data",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "32",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ',',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "terminal_actions",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "0",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ';',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
    ],
]
filter_raw: !http
pairs: Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "!http",
                                start: 0,
                                end: 5,
                            },
                            inner: [
                                Pair {
                                    rule: not_op,
                                    span: Span {
                                        str: "!",
                                        start: 0,
                                        end: 1,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "http",
                                        start: 1,
                                        end: 5,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: EOI,
            span: Span {
                str: "",
                start: 5,
                end: 5,
            },
            inner: [],
        },
    ],
)
pair: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "!http",
                    start: 0,
                    end: 5,
                },
                inner: [
                    Pair {
                        rule: not_op,
                        span: Span {
                            str: "!",
                            start: 0,
                            end: 1,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "http",
                            start: 1,
                            end: 5,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: not_op,
                    span: Span {
                        str: "!",
                        start: 0,
                        end: 1,
                    },
                    inner: [],
                },
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "http",
                        start: 1,
                        end: 5,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: not_op,
                span: Span {
                    str: "!",
                    start: 0,
                    end: 1,
                },
                inner: [],
            },
            Pair {
                rule: protocol,
                span: Span {
                    str: "http",
                    start: 1,
                    end: 5,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: not_op,
            span: Span {
                str: "!",
                start: 0,
                end: 1,
            },
            inner: [],
        },
        Pair {
            rule: protocol,
            span: Span {
                str: "http",
                start: 1,
                end: 5,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: not_op,
        span: Span {
            str: "!",
            start: 0,
            end: 1,
        },
        inner: [],
    },
    Pair {
        rule: protocol,
        span: Span {
            str: "http",
            start: 1,
            end: 5,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
            ),
        ],
    ),
]
ast: Disjunct(
    [
        Conjunct(
            [
                Predicate(
                    Unary {
                        protocol: ProtocolName(
                            "http",
                        ),
                    },
                ),
            ],
        ),
    ],
)
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ],
]
ptree: FlatPTree {
    root: FlatPNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        is_terminal: false,
        terminates: None,
        patterns: [
            0,
            1,
        ],
        children: [
            FlatPNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    0,
                ],
                children: [
                    FlatPNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            0,
                        ],
                        children: [
                            FlatPNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    0,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
            FlatPNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    1,
                ],
                children: [
                    FlatPNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            1,
                        ],
                        children: [
                            FlatPNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    1,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    size: 7,
}
filter_subtree: filter: Filter {
    patterns: [
        LayeredPattern(
            {
                ProtocolName(
                    "ipv4",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
        LayeredPattern(
            {
                ProtocolName(
                    "ipv6",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
    ],
}
filter_subtree: patterns: [
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv4",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv6",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
]
filter_subtree: deliver: Deliver {
    id: 0,
    as_str: "not_http_cb2(HttpTransaction)",
    must_deliver: false,
}
filter_subtree: ptree before collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [
                    PNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            0,
                        ],
                        children: [
                            PNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                actions: Actions {
                                    data: ActionData[SessionDeliver],
                                    terminal_actions: ActionData[],
                                },
                                deliver: {},
                                patterns: [
                                    0,
                                ],
                                children: [],
                                if_else: false,
                            },
                        ],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
            PNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    1,
                ],
                children: [
                    PNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            1,
                        ],
                        children: [
                            PNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                actions: Actions {
                                    data: ActionData[SessionDeliver],
                                    terminal_actions: ActionData[],
                                },
                                deliver: {},
                                patterns: [
                                    1,
                                ],
                                children: [],
                                if_else: false,
                            },
                        ],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
        ],
        if_else: false,
    },
    size: 7,
    actions: Actions {
        data: ActionData[SessionDeliver],
        terminal_actions: ActionData[],
    },
    filter_layer: Protocol,
    collapsed: false,
}
filter_subtree: ptree after collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
                actions: Actions {
                    data: ActionData[SessionDeliver],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [],
                if_else: false,
            },
        ],
        if_else: false,
    },
    size: 2,
    actions: Actions {
        data: ActionData[SessionDeliver],
        terminal_actions: ActionData[],
    },
    filter_layer: Protocol,
    collapsed: false,
}
conn_ptree: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
                actions: Actions {
                    data: ActionData[SessionDeliver],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [],
                if_else: false,
            },
        ],
        if_else: false,
    },
    size: 2,
    actions: Actions {
        data: ActionData[SessionDeliver],
        terminal_actions: ActionData[],
    },
    filter_layer: Protocol,
    collapsed: false,
}
gen_proto_filter: body: []
node: PNode {
    id: 1,
    pred: Unary {
        protocol: ProtocolName(
            "http",
        ),
    },
    actions: Actions {
        data: ActionData[SessionDeliver],
        terminal_actions: ActionData[],
    },
    deliver: {},
    patterns: [
        0,
    ],
    children: [],
    if_else: false,
}
body after build_child_nodes: []
update_body: body: [
    TokenStream [
        Ident {
            ident: "result",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '.',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "push",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Punct {
                    ch: '&',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "Actions",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [
                        Ident {
                            ident: "data",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "128",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ',',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "terminal_actions",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "0",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ';',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
    ],
]
body after update_body: [
    TokenStream [
        Ident {
            ident: "result",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '.',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "push",
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Punct {
                    ch: '&',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "Actions",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [
                        Ident {
                            ident: "data",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "128",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ',',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "terminal_actions",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "ActionData",
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: ':',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "from",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [
                                Literal {
                                    kind: Integer,
                                    symbol: "0",
                                    suffix: None,
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: ';',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
    ],
]
gen_proto_filter #2: body: [
    TokenStream [
        Ident {
            ident: "if",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '!',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Ident {
            ident: "matches",
            span: #56 bytes(1698..1715),
        },
        Punct {
            ch: '!',
            spacing: Alone,
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Parenthesis,
            stream: TokenStream [
                Ident {
                    ident: "conn",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: '.',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "service",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [],
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ',',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "retina_core",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Joint,
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "protocols",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Joint,
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "stream",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Joint,
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "ConnParser",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Joint,
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ':',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "Http",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [
                        Punct {
                            ch: '.',
                            spacing: Joint,
                            span: #56 bytes(1698..1715),
                        },
                        Punct {
                            ch: '.',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
        Group {
            delimiter: Brace,
            stream: TokenStream [
                Ident {
                    ident: "result",
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: '.',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
                Ident {
                    ident: "push",
                    span: #56 bytes(1698..1715),
                },
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [
                        Punct {
                            ch: '&',
                            spacing: Alone,
                            span: #56 bytes(1698..1715),
                        },
                        Ident {
                            ident: "Actions",
                            span: #56 bytes(1698..1715),
                        },
                        Group {
                            delimiter: Brace,
                            stream: TokenStream [
                                Ident {
                                    ident: "data",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "ActionData",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Joint,
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "from",
                                    span: #56 bytes(1698..1715),
                                },
                                Group {
                                    delimiter: Parenthesis,
                                    stream: TokenStream [
                                        Literal {
                                            kind: Integer,
                                            symbol: "128",
                                            suffix: None,
                                            span: #56 bytes(1698..1715),
                                        },
                                    ],
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ',',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "terminal_actions",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "ActionData",
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Joint,
                                    span: #56 bytes(1698..1715),
                                },
                                Punct {
                                    ch: ':',
                                    spacing: Alone,
                                    span: #56 bytes(1698..1715),
                                },
                                Ident {
                                    ident: "from",
                                    span: #56 bytes(1698..1715),
                                },
                                Group {
                                    delimiter: Parenthesis,
                                    stream: TokenStream [
                                        Literal {
                                            kind: Integer,
                                            symbol: "0",
                                            suffix: None,
                                            span: #56 bytes(1698..1715),
                                        },
                                    ],
                                    span: #56 bytes(1698..1715),
                                },
                            ],
                            span: #56 bytes(1698..1715),
                        },
                    ],
                    span: #56 bytes(1698..1715),
                },
                Punct {
                    ch: ';',
                    spacing: Alone,
                    span: #56 bytes(1698..1715),
                },
            ],
            span: #56 bytes(1698..1715),
        },
    ],
]
proto_filter: TokenStream [
    Ident {
        ident: "let",
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "mut",
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "result",
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: '=',
        spacing: Alone,
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "retina_core",
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: ':',
        spacing: Joint,
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: ':',
        spacing: Alone,
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "filter",
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: ':',
        spacing: Joint,
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: ':',
        spacing: Alone,
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "Actions",
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: ':',
        spacing: Joint,
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: ':',
        spacing: Alone,
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "new",
        span: #56 bytes(1698..1715),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: ';',
        spacing: Alone,
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "if",
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: '!',
        spacing: Alone,
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "matches",
        span: #56 bytes(1698..1715),
    },
    Punct {
        ch: '!',
        spacing: Alone,
        span: #56 bytes(1698..1715),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Ident {
                ident: "conn",
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: '.',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
            Ident {
                ident: "service",
                span: #56 bytes(1698..1715),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [],
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ',',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
            Ident {
                ident: "retina_core",
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Joint,
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
            Ident {
                ident: "protocols",
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Joint,
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
            Ident {
                ident: "stream",
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Joint,
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
            Ident {
                ident: "ConnParser",
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Joint,
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
            Ident {
                ident: "Http",
                span: #56 bytes(1698..1715),
            },
            Group {
                delimiter: Brace,
                stream: TokenStream [
                    Punct {
                        ch: '.',
                        spacing: Joint,
                        span: #56 bytes(1698..1715),
                    },
                    Punct {
                        ch: '.',
                        spacing: Alone,
                        span: #56 bytes(1698..1715),
                    },
                ],
                span: #56 bytes(1698..1715),
            },
        ],
        span: #56 bytes(1698..1715),
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "result",
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: '.',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
            Ident {
                ident: "push",
                span: #56 bytes(1698..1715),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [
                    Punct {
                        ch: '&',
                        spacing: Alone,
                        span: #56 bytes(1698..1715),
                    },
                    Ident {
                        ident: "Actions",
                        span: #56 bytes(1698..1715),
                    },
                    Group {
                        delimiter: Brace,
                        stream: TokenStream [
                            Ident {
                                ident: "data",
                                span: #56 bytes(1698..1715),
                            },
                            Punct {
                                ch: ':',
                                spacing: Alone,
                                span: #56 bytes(1698..1715),
                            },
                            Ident {
                                ident: "ActionData",
                                span: #56 bytes(1698..1715),
                            },
                            Punct {
                                ch: ':',
                                spacing: Joint,
                                span: #56 bytes(1698..1715),
                            },
                            Punct {
                                ch: ':',
                                spacing: Alone,
                                span: #56 bytes(1698..1715),
                            },
                            Ident {
                                ident: "from",
                                span: #56 bytes(1698..1715),
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Literal {
                                        kind: Integer,
                                        symbol: "128",
                                        suffix: None,
                                        span: #56 bytes(1698..1715),
                                    },
                                ],
                                span: #56 bytes(1698..1715),
                            },
                            Punct {
                                ch: ',',
                                spacing: Alone,
                                span: #56 bytes(1698..1715),
                            },
                            Ident {
                                ident: "terminal_actions",
                                span: #56 bytes(1698..1715),
                            },
                            Punct {
                                ch: ':',
                                spacing: Alone,
                                span: #56 bytes(1698..1715),
                            },
                            Ident {
                                ident: "ActionData",
                                span: #56 bytes(1698..1715),
                            },
                            Punct {
                                ch: ':',
                                spacing: Joint,
                                span: #56 bytes(1698..1715),
                            },
                            Punct {
                                ch: ':',
                                spacing: Alone,
                                span: #56 bytes(1698..1715),
                            },
                            Ident {
                                ident: "from",
                                span: #56 bytes(1698..1715),
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Literal {
                                        kind: Integer,
                                        symbol: "0",
                                        suffix: None,
                                        span: #56 bytes(1698..1715),
                                    },
                                ],
                                span: #56 bytes(1698..1715),
                            },
                        ],
                        span: #56 bytes(1698..1715),
                    },
                ],
                span: #56 bytes(1698..1715),
            },
            Punct {
                ch: ';',
                spacing: Alone,
                span: #56 bytes(1698..1715),
            },
        ],
        span: #56 bytes(1698..1715),
    },
    Ident {
        ident: "result",
        span: #56 bytes(1698..1715),
    },
]
filter_raw: !http
pairs: Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "!http",
                                start: 0,
                                end: 5,
                            },
                            inner: [
                                Pair {
                                    rule: not_op,
                                    span: Span {
                                        str: "!",
                                        start: 0,
                                        end: 1,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "http",
                                        start: 1,
                                        end: 5,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: EOI,
            span: Span {
                str: "",
                start: 5,
                end: 5,
            },
            inner: [],
        },
    ],
)
pair: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "!http",
                    start: 0,
                    end: 5,
                },
                inner: [
                    Pair {
                        rule: not_op,
                        span: Span {
                            str: "!",
                            start: 0,
                            end: 1,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "http",
                            start: 1,
                            end: 5,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: not_op,
                    span: Span {
                        str: "!",
                        start: 0,
                        end: 1,
                    },
                    inner: [],
                },
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "http",
                        start: 1,
                        end: 5,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: not_op,
                span: Span {
                    str: "!",
                    start: 0,
                    end: 1,
                },
                inner: [],
            },
            Pair {
                rule: protocol,
                span: Span {
                    str: "http",
                    start: 1,
                    end: 5,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: not_op,
            span: Span {
                str: "!",
                start: 0,
                end: 1,
            },
            inner: [],
        },
        Pair {
            rule: protocol,
            span: Span {
                str: "http",
                start: 1,
                end: 5,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: not_op,
        span: Span {
            str: "!",
            start: 0,
            end: 1,
        },
        inner: [],
    },
    Pair {
        rule: protocol,
        span: Span {
            str: "http",
            start: 1,
            end: 5,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
            ),
        ],
    ),
]
ast: Disjunct(
    [
        Conjunct(
            [
                Predicate(
                    Unary {
                        protocol: ProtocolName(
                            "http",
                        ),
                    },
                ),
            ],
        ),
    ],
)
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ],
]
ptree: FlatPTree {
    root: FlatPNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        is_terminal: false,
        terminates: None,
        patterns: [
            0,
            1,
        ],
        children: [
            FlatPNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    0,
                ],
                children: [
                    FlatPNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            0,
                        ],
                        children: [
                            FlatPNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    0,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
            FlatPNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    1,
                ],
                children: [
                    FlatPNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            1,
                        ],
                        children: [
                            FlatPNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    1,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    size: 7,
}
filter_subtree: filter: Filter {
    patterns: [
        LayeredPattern(
            {
                ProtocolName(
                    "ipv4",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
        LayeredPattern(
            {
                ProtocolName(
                    "ipv6",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
    ],
}
filter_subtree: patterns: [
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv4",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv6",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
]
filter_subtree: deliver: Deliver {
    id: 0,
    as_str: "not_http_cb2(HttpTransaction)",
    must_deliver: false,
}
filter_subtree: ptree before collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    0,
                ],
                children: [
                    PNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            0,
                        ],
                        children: [
                            PNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                actions: Actions {
                                    data: ActionData[],
                                    terminal_actions: ActionData[],
                                },
                                deliver: {
                                    Deliver {
                                        id: 0,
                                        as_str: "not_http_cb2(HttpTransaction)",
                                        must_deliver: false,
                                    },
                                },
                                patterns: [
                                    0,
                                ],
                                children: [],
                                if_else: false,
                            },
                        ],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
            PNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {},
                patterns: [
                    1,
                ],
                children: [
                    PNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        actions: Actions {
                            data: ActionData[],
                            terminal_actions: ActionData[],
                        },
                        deliver: {},
                        patterns: [
                            1,
                        ],
                        children: [
                            PNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                actions: Actions {
                                    data: ActionData[],
                                    terminal_actions: ActionData[],
                                },
                                deliver: {
                                    Deliver {
                                        id: 0,
                                        as_str: "not_http_cb2(HttpTransaction)",
                                        must_deliver: false,
                                    },
                                },
                                patterns: [
                                    1,
                                ],
                                children: [],
                                if_else: false,
                            },
                        ],
                        if_else: false,
                    },
                ],
                if_else: false,
            },
        ],
        if_else: false,
    },
    size: 7,
    actions: Actions {
        data: ActionData[],
        terminal_actions: ActionData[],
    },
    filter_layer: Session,
    collapsed: false,
}
filter_subtree: ptree after collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [
            0,
            1,
        ],
        children: [
            PNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
                actions: Actions {
                    data: ActionData[],
                    terminal_actions: ActionData[],
                },
                deliver: {
                    Deliver {
                        id: 0,
                        as_str: "not_http_cb2(HttpTransaction)",
                        must_deliver: false,
                    },
                },
                patterns: [
                    0,
                ],
                children: [],
                if_else: false,
            },
        ],
        if_else: false,
    },
    size: 2,
    actions: Actions {
        data: ActionData[],
        terminal_actions: ActionData[],
    },
    filter_layer: Session,
    collapsed: false,
}
update_body: body: []
filter_raw: !http
pairs: Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "!http",
                                start: 0,
                                end: 5,
                            },
                            inner: [
                                Pair {
                                    rule: not_op,
                                    span: Span {
                                        str: "!",
                                        start: 0,
                                        end: 1,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "http",
                                        start: 1,
                                        end: 5,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: EOI,
            span: Span {
                str: "",
                start: 5,
                end: 5,
            },
            inner: [],
        },
    ],
)
pair: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "!http",
                    start: 0,
                    end: 5,
                },
                inner: [
                    Pair {
                        rule: not_op,
                        span: Span {
                            str: "!",
                            start: 0,
                            end: 1,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "http",
                            start: 1,
                            end: 5,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: not_op,
                    span: Span {
                        str: "!",
                        start: 0,
                        end: 1,
                    },
                    inner: [],
                },
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "http",
                        start: 1,
                        end: 5,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: not_op,
                span: Span {
                    str: "!",
                    start: 0,
                    end: 1,
                },
                inner: [],
            },
            Pair {
                rule: protocol,
                span: Span {
                    str: "http",
                    start: 1,
                    end: 5,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: not_op,
            span: Span {
                str: "!",
                start: 0,
                end: 1,
            },
            inner: [],
        },
        Pair {
            rule: protocol,
            span: Span {
                str: "http",
                start: 1,
                end: 5,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: not_op,
        span: Span {
            str: "!",
            start: 0,
            end: 1,
        },
        inner: [],
    },
    Pair {
        rule: protocol,
        span: Span {
            str: "http",
            start: 1,
            end: 5,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
            ),
        ],
    ),
]
ast: Disjunct(
    [
        Conjunct(
            [
                Predicate(
                    Unary {
                        protocol: ProtocolName(
                            "http",
                        ),
                    },
                ),
            ],
        ),
    ],
)
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ],
]
ptree: FlatPTree {
    root: FlatPNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        is_terminal: false,
        terminates: None,
        patterns: [
            0,
            1,
        ],
        children: [
            FlatPNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    0,
                ],
                children: [
                    FlatPNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            0,
                        ],
                        children: [
                            FlatPNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    0,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
            FlatPNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    1,
                ],
                children: [
                    FlatPNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            1,
                        ],
                        children: [
                            FlatPNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    1,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    size: 7,
}
filter_subtree: filter: Filter {
    patterns: [
        LayeredPattern(
            {
                ProtocolName(
                    "ipv4",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
        LayeredPattern(
            {
                ProtocolName(
                    "ipv6",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
    ],
}
filter_subtree: patterns: [
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv4",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv6",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
]
filter_subtree: deliver: Deliver {
    id: 0,
    as_str: "not_http_cb2(HttpTransaction)",
    must_deliver: false,
}
filter_subtree: ptree before collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [],
        children: [],
        if_else: false,
    },
    size: 1,
    actions: Actions {
        data: ActionData[],
        terminal_actions: ActionData[],
    },
    filter_layer: ConnectionDeliver,
    collapsed: false,
}
filter_subtree: ptree after collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [],
        children: [],
        if_else: false,
    },
    size: 1,
    actions: Actions {
        data: ActionData[],
        terminal_actions: ActionData[],
    },
    filter_layer: ConnectionDeliver,
    collapsed: true,
}
conn_deliver_ptree: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [],
        children: [],
        if_else: false,
    },
    size: 1,
    actions: Actions {
        data: ActionData[],
        terminal_actions: ActionData[],
    },
    filter_layer: ConnectionDeliver,
    collapsed: true,
}
filter_raw: !http
pairs: Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "!http",
                                start: 0,
                                end: 5,
                            },
                            inner: [
                                Pair {
                                    rule: not_op,
                                    span: Span {
                                        str: "!",
                                        start: 0,
                                        end: 1,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "http",
                                        start: 1,
                                        end: 5,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: EOI,
            span: Span {
                str: "",
                start: 5,
                end: 5,
            },
            inner: [],
        },
    ],
)
pair: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "!http",
                        start: 0,
                        end: 5,
                    },
                    inner: [
                        Pair {
                            rule: not_op,
                            span: Span {
                                str: "!",
                                start: 0,
                                end: 1,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "http",
                                start: 1,
                                end: 5,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "!http",
                    start: 0,
                    end: 5,
                },
                inner: [
                    Pair {
                        rule: not_op,
                        span: Span {
                            str: "!",
                            start: 0,
                            end: 1,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "http",
                            start: 1,
                            end: 5,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "!http",
                start: 0,
                end: 5,
            },
            inner: [
                Pair {
                    rule: not_op,
                    span: Span {
                        str: "!",
                        start: 0,
                        end: 1,
                    },
                    inner: [],
                },
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "http",
                        start: 1,
                        end: 5,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "!http",
            start: 0,
            end: 5,
        },
        inner: [
            Pair {
                rule: not_op,
                span: Span {
                    str: "!",
                    start: 0,
                    end: 1,
                },
                inner: [],
            },
            Pair {
                rule: protocol,
                span: Span {
                    str: "http",
                    start: 1,
                    end: 5,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "!http",
        start: 0,
        end: 5,
    },
    inner: [
        Pair {
            rule: not_op,
            span: Span {
                str: "!",
                start: 0,
                end: 1,
            },
            inner: [],
        },
        Pair {
            rule: protocol,
            span: Span {
                str: "http",
                start: 1,
                end: 5,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: not_op,
        span: Span {
            str: "!",
            start: 0,
            end: 1,
        },
        inner: [],
    },
    Pair {
        rule: protocol,
        span: Span {
            str: "http",
            start: 1,
            end: 5,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "http",
        start: 1,
        end: 5,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "http",
                    ),
                },
            ),
        ],
    ),
]
ast: Disjunct(
    [
        Conjunct(
            [
                Predicate(
                    Unary {
                        protocol: ProtocolName(
                            "http",
                        ),
                    },
                ),
            ],
        ),
    ],
)
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "http",
            ),
        },
    ],
]
ptree: FlatPTree {
    root: FlatPNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        is_terminal: false,
        terminates: None,
        patterns: [
            0,
            1,
        ],
        children: [
            FlatPNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    0,
                ],
                children: [
                    FlatPNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            0,
                        ],
                        children: [
                            FlatPNode {
                                id: 3,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    0,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
            FlatPNode {
                id: 4,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    1,
                ],
                children: [
                    FlatPNode {
                        id: 5,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: false,
                        terminates: Packet,
                        patterns: [
                            1,
                        ],
                        children: [
                            FlatPNode {
                                id: 6,
                                pred: Unary {
                                    protocol: ProtocolName(
                                        "http",
                                    ),
                                },
                                is_terminal: true,
                                terminates: Connection,
                                patterns: [
                                    1,
                                ],
                                children: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    size: 7,
}
filter_subtree: filter: Filter {
    patterns: [
        LayeredPattern(
            {
                ProtocolName(
                    "ipv4",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
        LayeredPattern(
            {
                ProtocolName(
                    "ipv6",
                ): [],
                ProtocolName(
                    "tcp",
                ): [],
                ProtocolName(
                    "http",
                ): [],
            },
        ),
    ],
}
filter_subtree: patterns: [
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv4",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
    FlatPattern {
        predicates: [
            Unary {
                protocol: ProtocolName(
                    "ipv6",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "tcp",
                ),
            },
            Unary {
                protocol: ProtocolName(
                    "http",
                ),
            },
        ],
    },
]
filter_subtree: deliver: Deliver {
    id: 0,
    as_str: "not_http_cb2(HttpTransaction)",
    must_deliver: false,
}
filter_subtree: ptree before collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [],
        children: [],
        if_else: false,
    },
    size: 1,
    actions: Actions {
        data: ActionData[],
        terminal_actions: ActionData[],
    },
    filter_layer: PacketDeliver,
    collapsed: false,
}
filter_subtree: ptree after collapse: PTree {
    root: PNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        actions: Actions {
            data: ActionData[],
            terminal_actions: ActionData[],
        },
        deliver: {},
        patterns: [],
        children: [],
        if_else: false,
    },
    size: 1,
    actions: Actions {
        data: ActionData[],
        terminal_actions: ActionData[],
    },
    filter_layer: PacketDeliver,
    collapsed: true,
}
Datatypes {
  HttpTransaction,
}

Parsers {
  http,
}

filter_raw: ((ipv4) and (tcp)) or ((ipv6) and (tcp))
pairs: Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "((ipv4) and (tcp)) or ((ipv6) and (tcp))",
                start: 0,
                end: 40,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "((ipv4) and (tcp)) ",
                        start: 0,
                        end: 19,
                    },
                    inner: [
                        Pair {
                            rule: expr,
                            span: Span {
                                str: "(ipv4) and (tcp)",
                                start: 1,
                                end: 17,
                            },
                            inner: [
                                Pair {
                                    rule: sub_expr,
                                    span: Span {
                                        str: "(ipv4) and (tcp)",
                                        start: 1,
                                        end: 17,
                                    },
                                    inner: [
                                        Pair {
                                            rule: expr,
                                            span: Span {
                                                str: "ipv4",
                                                start: 2,
                                                end: 6,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: sub_expr,
                                                    span: Span {
                                                        str: "ipv4",
                                                        start: 2,
                                                        end: 6,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: predicate,
                                                            span: Span {
                                                                str: "ipv4",
                                                                start: 2,
                                                                end: 6,
                                                            },
                                                            inner: [
                                                                Pair {
                                                                    rule: protocol,
                                                                    span: Span {
                                                                        str: "ipv4",
                                                                        start: 2,
                                                                        end: 6,
                                                                    },
                                                                    inner: [],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        Pair {
                                            rule: and_op,
                                            span: Span {
                                                str: "and",
                                                start: 8,
                                                end: 11,
                                            },
                                            inner: [],
                                        },
                                        Pair {
                                            rule: expr,
                                            span: Span {
                                                str: "tcp",
                                                start: 13,
                                                end: 16,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: sub_expr,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 13,
                                                        end: 16,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: predicate,
                                                            span: Span {
                                                                str: "tcp",
                                                                start: 13,
                                                                end: 16,
                                                            },
                                                            inner: [
                                                                Pair {
                                                                    rule: protocol,
                                                                    span: Span {
                                                                        str: "tcp",
                                                                        start: 13,
                                                                        end: 16,
                                                                    },
                                                                    inner: [],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
                Pair {
                    rule: or_op,
                    span: Span {
                        str: "or",
                        start: 19,
                        end: 21,
                    },
                    inner: [],
                },
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "((ipv6) and (tcp))",
                        start: 22,
                        end: 40,
                    },
                    inner: [
                        Pair {
                            rule: expr,
                            span: Span {
                                str: "(ipv6) and (tcp)",
                                start: 23,
                                end: 39,
                            },
                            inner: [
                                Pair {
                                    rule: sub_expr,
                                    span: Span {
                                        str: "(ipv6) and (tcp)",
                                        start: 23,
                                        end: 39,
                                    },
                                    inner: [
                                        Pair {
                                            rule: expr,
                                            span: Span {
                                                str: "ipv6",
                                                start: 24,
                                                end: 28,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: sub_expr,
                                                    span: Span {
                                                        str: "ipv6",
                                                        start: 24,
                                                        end: 28,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: predicate,
                                                            span: Span {
                                                                str: "ipv6",
                                                                start: 24,
                                                                end: 28,
                                                            },
                                                            inner: [
                                                                Pair {
                                                                    rule: protocol,
                                                                    span: Span {
                                                                        str: "ipv6",
                                                                        start: 24,
                                                                        end: 28,
                                                                    },
                                                                    inner: [],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        Pair {
                                            rule: and_op,
                                            span: Span {
                                                str: "and",
                                                start: 30,
                                                end: 33,
                                            },
                                            inner: [],
                                        },
                                        Pair {
                                            rule: expr,
                                            span: Span {
                                                str: "tcp",
                                                start: 35,
                                                end: 38,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: sub_expr,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 35,
                                                        end: 38,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: predicate,
                                                            span: Span {
                                                                str: "tcp",
                                                                start: 35,
                                                                end: 38,
                                                            },
                                                            inner: [
                                                                Pair {
                                                                    rule: protocol,
                                                                    span: Span {
                                                                        str: "tcp",
                                                                        start: 35,
                                                                        end: 38,
                                                                    },
                                                                    inner: [],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: EOI,
            span: Span {
                str: "",
                start: 40,
                end: 40,
            },
            inner: [],
        },
    ],
)
pair: Pair {
    rule: expr,
    span: Span {
        str: "((ipv4) and (tcp)) or ((ipv6) and (tcp))",
        start: 0,
        end: 40,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "((ipv4) and (tcp)) ",
                start: 0,
                end: 19,
            },
            inner: [
                Pair {
                    rule: expr,
                    span: Span {
                        str: "(ipv4) and (tcp)",
                        start: 1,
                        end: 17,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "(ipv4) and (tcp)",
                                start: 1,
                                end: 17,
                            },
                            inner: [
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "ipv4",
                                        start: 2,
                                        end: 6,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "ipv4",
                                                start: 2,
                                                end: 6,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "ipv4",
                                                        start: 2,
                                                        end: 6,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "ipv4",
                                                                start: 2,
                                                                end: 6,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                Pair {
                                    rule: and_op,
                                    span: Span {
                                        str: "and",
                                        start: 8,
                                        end: 11,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "tcp",
                                        start: 13,
                                        end: 16,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "tcp",
                                                start: 13,
                                                end: 16,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 13,
                                                        end: 16,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "tcp",
                                                                start: 13,
                                                                end: 16,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: or_op,
            span: Span {
                str: "or",
                start: 19,
                end: 21,
            },
            inner: [],
        },
        Pair {
            rule: sub_expr,
            span: Span {
                str: "((ipv6) and (tcp))",
                start: 22,
                end: 40,
            },
            inner: [
                Pair {
                    rule: expr,
                    span: Span {
                        str: "(ipv6) and (tcp)",
                        start: 23,
                        end: 39,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "(ipv6) and (tcp)",
                                start: 23,
                                end: 39,
                            },
                            inner: [
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "ipv6",
                                        start: 24,
                                        end: 28,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "ipv6",
                                                start: 24,
                                                end: 28,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "ipv6",
                                                        start: 24,
                                                        end: 28,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "ipv6",
                                                                start: 24,
                                                                end: 28,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                Pair {
                                    rule: and_op,
                                    span: Span {
                                        str: "and",
                                        start: 30,
                                        end: 33,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "tcp",
                                        start: 35,
                                        end: 38,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "tcp",
                                                start: 35,
                                                end: 38,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 35,
                                                        end: 38,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "tcp",
                                                                start: 35,
                                                                end: 38,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "((ipv4) and (tcp)) or ((ipv6) and (tcp))",
        start: 0,
        end: 40,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "((ipv4) and (tcp)) ",
                start: 0,
                end: 19,
            },
            inner: [
                Pair {
                    rule: expr,
                    span: Span {
                        str: "(ipv4) and (tcp)",
                        start: 1,
                        end: 17,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "(ipv4) and (tcp)",
                                start: 1,
                                end: 17,
                            },
                            inner: [
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "ipv4",
                                        start: 2,
                                        end: 6,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "ipv4",
                                                start: 2,
                                                end: 6,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "ipv4",
                                                        start: 2,
                                                        end: 6,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "ipv4",
                                                                start: 2,
                                                                end: 6,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                Pair {
                                    rule: and_op,
                                    span: Span {
                                        str: "and",
                                        start: 8,
                                        end: 11,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "tcp",
                                        start: 13,
                                        end: 16,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "tcp",
                                                start: 13,
                                                end: 16,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 13,
                                                        end: 16,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "tcp",
                                                                start: 13,
                                                                end: 16,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: or_op,
            span: Span {
                str: "or",
                start: 19,
                end: 21,
            },
            inner: [],
        },
        Pair {
            rule: sub_expr,
            span: Span {
                str: "((ipv6) and (tcp))",
                start: 22,
                end: 40,
            },
            inner: [
                Pair {
                    rule: expr,
                    span: Span {
                        str: "(ipv6) and (tcp)",
                        start: 23,
                        end: 39,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "(ipv6) and (tcp)",
                                start: 23,
                                end: 39,
                            },
                            inner: [
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "ipv6",
                                        start: 24,
                                        end: 28,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "ipv6",
                                                start: 24,
                                                end: 28,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "ipv6",
                                                        start: 24,
                                                        end: 28,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "ipv6",
                                                                start: 24,
                                                                end: 28,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                Pair {
                                    rule: and_op,
                                    span: Span {
                                        str: "and",
                                        start: 30,
                                        end: 33,
                                    },
                                    inner: [],
                                },
                                Pair {
                                    rule: expr,
                                    span: Span {
                                        str: "tcp",
                                        start: 35,
                                        end: 38,
                                    },
                                    inner: [
                                        Pair {
                                            rule: sub_expr,
                                            span: Span {
                                                str: "tcp",
                                                start: 35,
                                                end: 38,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: predicate,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 35,
                                                        end: 38,
                                                    },
                                                    inner: [
                                                        Pair {
                                                            rule: protocol,
                                                            span: Span {
                                                                str: "tcp",
                                                                start: 35,
                                                                end: 38,
                                                            },
                                                            inner: [],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "((ipv4) and (tcp)) ",
            start: 0,
            end: 19,
        },
        inner: [
            Pair {
                rule: expr,
                span: Span {
                    str: "(ipv4) and (tcp)",
                    start: 1,
                    end: 17,
                },
                inner: [
                    Pair {
                        rule: sub_expr,
                        span: Span {
                            str: "(ipv4) and (tcp)",
                            start: 1,
                            end: 17,
                        },
                        inner: [
                            Pair {
                                rule: expr,
                                span: Span {
                                    str: "ipv4",
                                    start: 2,
                                    end: 6,
                                },
                                inner: [
                                    Pair {
                                        rule: sub_expr,
                                        span: Span {
                                            str: "ipv4",
                                            start: 2,
                                            end: 6,
                                        },
                                        inner: [
                                            Pair {
                                                rule: predicate,
                                                span: Span {
                                                    str: "ipv4",
                                                    start: 2,
                                                    end: 6,
                                                },
                                                inner: [
                                                    Pair {
                                                        rule: protocol,
                                                        span: Span {
                                                            str: "ipv4",
                                                            start: 2,
                                                            end: 6,
                                                        },
                                                        inner: [],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                            Pair {
                                rule: and_op,
                                span: Span {
                                    str: "and",
                                    start: 8,
                                    end: 11,
                                },
                                inner: [],
                            },
                            Pair {
                                rule: expr,
                                span: Span {
                                    str: "tcp",
                                    start: 13,
                                    end: 16,
                                },
                                inner: [
                                    Pair {
                                        rule: sub_expr,
                                        span: Span {
                                            str: "tcp",
                                            start: 13,
                                            end: 16,
                                        },
                                        inner: [
                                            Pair {
                                                rule: predicate,
                                                span: Span {
                                                    str: "tcp",
                                                    start: 13,
                                                    end: 16,
                                                },
                                                inner: [
                                                    Pair {
                                                        rule: protocol,
                                                        span: Span {
                                                            str: "tcp",
                                                            start: 13,
                                                            end: 16,
                                                        },
                                                        inner: [],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Pair {
        rule: or_op,
        span: Span {
            str: "or",
            start: 19,
            end: 21,
        },
        inner: [],
    },
    Pair {
        rule: sub_expr,
        span: Span {
            str: "((ipv6) and (tcp))",
            start: 22,
            end: 40,
        },
        inner: [
            Pair {
                rule: expr,
                span: Span {
                    str: "(ipv6) and (tcp)",
                    start: 23,
                    end: 39,
                },
                inner: [
                    Pair {
                        rule: sub_expr,
                        span: Span {
                            str: "(ipv6) and (tcp)",
                            start: 23,
                            end: 39,
                        },
                        inner: [
                            Pair {
                                rule: expr,
                                span: Span {
                                    str: "ipv6",
                                    start: 24,
                                    end: 28,
                                },
                                inner: [
                                    Pair {
                                        rule: sub_expr,
                                        span: Span {
                                            str: "ipv6",
                                            start: 24,
                                            end: 28,
                                        },
                                        inner: [
                                            Pair {
                                                rule: predicate,
                                                span: Span {
                                                    str: "ipv6",
                                                    start: 24,
                                                    end: 28,
                                                },
                                                inner: [
                                                    Pair {
                                                        rule: protocol,
                                                        span: Span {
                                                            str: "ipv6",
                                                            start: 24,
                                                            end: 28,
                                                        },
                                                        inner: [],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                            Pair {
                                rule: and_op,
                                span: Span {
                                    str: "and",
                                    start: 30,
                                    end: 33,
                                },
                                inner: [],
                            },
                            Pair {
                                rule: expr,
                                span: Span {
                                    str: "tcp",
                                    start: 35,
                                    end: 38,
                                },
                                inner: [
                                    Pair {
                                        rule: sub_expr,
                                        span: Span {
                                            str: "tcp",
                                            start: 35,
                                            end: 38,
                                        },
                                        inner: [
                                            Pair {
                                                rule: predicate,
                                                span: Span {
                                                    str: "tcp",
                                                    start: 35,
                                                    end: 38,
                                                },
                                                inner: [
                                                    Pair {
                                                        rule: protocol,
                                                        span: Span {
                                                            str: "tcp",
                                                            start: 35,
                                                            end: 38,
                                                        },
                                                        inner: [],
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "((ipv4) and (tcp)) ",
        start: 0,
        end: 19,
    },
    inner: [
        Pair {
            rule: expr,
            span: Span {
                str: "(ipv4) and (tcp)",
                start: 1,
                end: 17,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "(ipv4) and (tcp)",
                        start: 1,
                        end: 17,
                    },
                    inner: [
                        Pair {
                            rule: expr,
                            span: Span {
                                str: "ipv4",
                                start: 2,
                                end: 6,
                            },
                            inner: [
                                Pair {
                                    rule: sub_expr,
                                    span: Span {
                                        str: "ipv4",
                                        start: 2,
                                        end: 6,
                                    },
                                    inner: [
                                        Pair {
                                            rule: predicate,
                                            span: Span {
                                                str: "ipv4",
                                                start: 2,
                                                end: 6,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: protocol,
                                                    span: Span {
                                                        str: "ipv4",
                                                        start: 2,
                                                        end: 6,
                                                    },
                                                    inner: [],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                        Pair {
                            rule: and_op,
                            span: Span {
                                str: "and",
                                start: 8,
                                end: 11,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: expr,
                            span: Span {
                                str: "tcp",
                                start: 13,
                                end: 16,
                            },
                            inner: [
                                Pair {
                                    rule: sub_expr,
                                    span: Span {
                                        str: "tcp",
                                        start: 13,
                                        end: 16,
                                    },
                                    inner: [
                                        Pair {
                                            rule: predicate,
                                            span: Span {
                                                str: "tcp",
                                                start: 13,
                                                end: 16,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: protocol,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 13,
                                                        end: 16,
                                                    },
                                                    inner: [],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: expr,
        span: Span {
            str: "(ipv4) and (tcp)",
            start: 1,
            end: 17,
        },
        inner: [
            Pair {
                rule: sub_expr,
                span: Span {
                    str: "(ipv4) and (tcp)",
                    start: 1,
                    end: 17,
                },
                inner: [
                    Pair {
                        rule: expr,
                        span: Span {
                            str: "ipv4",
                            start: 2,
                            end: 6,
                        },
                        inner: [
                            Pair {
                                rule: sub_expr,
                                span: Span {
                                    str: "ipv4",
                                    start: 2,
                                    end: 6,
                                },
                                inner: [
                                    Pair {
                                        rule: predicate,
                                        span: Span {
                                            str: "ipv4",
                                            start: 2,
                                            end: 6,
                                        },
                                        inner: [
                                            Pair {
                                                rule: protocol,
                                                span: Span {
                                                    str: "ipv4",
                                                    start: 2,
                                                    end: 6,
                                                },
                                                inner: [],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                    Pair {
                        rule: and_op,
                        span: Span {
                            str: "and",
                            start: 8,
                            end: 11,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: expr,
                        span: Span {
                            str: "tcp",
                            start: 13,
                            end: 16,
                        },
                        inner: [
                            Pair {
                                rule: sub_expr,
                                span: Span {
                                    str: "tcp",
                                    start: 13,
                                    end: 16,
                                },
                                inner: [
                                    Pair {
                                        rule: predicate,
                                        span: Span {
                                            str: "tcp",
                                            start: 13,
                                            end: 16,
                                        },
                                        inner: [
                                            Pair {
                                                rule: protocol,
                                                span: Span {
                                                    str: "tcp",
                                                    start: 13,
                                                    end: 16,
                                                },
                                                inner: [],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
]
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "(ipv4) and (tcp)",
        start: 1,
        end: 17,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "(ipv4) and (tcp)",
                start: 1,
                end: 17,
            },
            inner: [
                Pair {
                    rule: expr,
                    span: Span {
                        str: "ipv4",
                        start: 2,
                        end: 6,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "ipv4",
                                start: 2,
                                end: 6,
                            },
                            inner: [
                                Pair {
                                    rule: predicate,
                                    span: Span {
                                        str: "ipv4",
                                        start: 2,
                                        end: 6,
                                    },
                                    inner: [
                                        Pair {
                                            rule: protocol,
                                            span: Span {
                                                str: "ipv4",
                                                start: 2,
                                                end: 6,
                                            },
                                            inner: [],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
                Pair {
                    rule: and_op,
                    span: Span {
                        str: "and",
                        start: 8,
                        end: 11,
                    },
                    inner: [],
                },
                Pair {
                    rule: expr,
                    span: Span {
                        str: "tcp",
                        start: 13,
                        end: 16,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "tcp",
                                start: 13,
                                end: 16,
                            },
                            inner: [
                                Pair {
                                    rule: predicate,
                                    span: Span {
                                        str: "tcp",
                                        start: 13,
                                        end: 16,
                                    },
                                    inner: [
                                        Pair {
                                            rule: protocol,
                                            span: Span {
                                                str: "tcp",
                                                start: 13,
                                                end: 16,
                                            },
                                            inner: [],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "(ipv4) and (tcp)",
            start: 1,
            end: 17,
        },
        inner: [
            Pair {
                rule: expr,
                span: Span {
                    str: "ipv4",
                    start: 2,
                    end: 6,
                },
                inner: [
                    Pair {
                        rule: sub_expr,
                        span: Span {
                            str: "ipv4",
                            start: 2,
                            end: 6,
                        },
                        inner: [
                            Pair {
                                rule: predicate,
                                span: Span {
                                    str: "ipv4",
                                    start: 2,
                                    end: 6,
                                },
                                inner: [
                                    Pair {
                                        rule: protocol,
                                        span: Span {
                                            str: "ipv4",
                                            start: 2,
                                            end: 6,
                                        },
                                        inner: [],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Pair {
                rule: and_op,
                span: Span {
                    str: "and",
                    start: 8,
                    end: 11,
                },
                inner: [],
            },
            Pair {
                rule: expr,
                span: Span {
                    str: "tcp",
                    start: 13,
                    end: 16,
                },
                inner: [
                    Pair {
                        rule: sub_expr,
                        span: Span {
                            str: "tcp",
                            start: 13,
                            end: 16,
                        },
                        inner: [
                            Pair {
                                rule: predicate,
                                span: Span {
                                    str: "tcp",
                                    start: 13,
                                    end: 16,
                                },
                                inner: [
                                    Pair {
                                        rule: protocol,
                                        span: Span {
                                            str: "tcp",
                                            start: 13,
                                            end: 16,
                                        },
                                        inner: [],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "(ipv4) and (tcp)",
        start: 1,
        end: 17,
    },
    inner: [
        Pair {
            rule: expr,
            span: Span {
                str: "ipv4",
                start: 2,
                end: 6,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "ipv4",
                        start: 2,
                        end: 6,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "ipv4",
                                start: 2,
                                end: 6,
                            },
                            inner: [
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "ipv4",
                                        start: 2,
                                        end: 6,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: and_op,
            span: Span {
                str: "and",
                start: 8,
                end: 11,
            },
            inner: [],
        },
        Pair {
            rule: expr,
            span: Span {
                str: "tcp",
                start: 13,
                end: 16,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "tcp",
                        start: 13,
                        end: 16,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "tcp",
                                start: 13,
                                end: 16,
                            },
                            inner: [
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "tcp",
                                        start: 13,
                                        end: 16,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: expr,
        span: Span {
            str: "ipv4",
            start: 2,
            end: 6,
        },
        inner: [
            Pair {
                rule: sub_expr,
                span: Span {
                    str: "ipv4",
                    start: 2,
                    end: 6,
                },
                inner: [
                    Pair {
                        rule: predicate,
                        span: Span {
                            str: "ipv4",
                            start: 2,
                            end: 6,
                        },
                        inner: [
                            Pair {
                                rule: protocol,
                                span: Span {
                                    str: "ipv4",
                                    start: 2,
                                    end: 6,
                                },
                                inner: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Pair {
        rule: and_op,
        span: Span {
            str: "and",
            start: 8,
            end: 11,
        },
        inner: [],
    },
    Pair {
        rule: expr,
        span: Span {
            str: "tcp",
            start: 13,
            end: 16,
        },
        inner: [
            Pair {
                rule: sub_expr,
                span: Span {
                    str: "tcp",
                    start: 13,
                    end: 16,
                },
                inner: [
                    Pair {
                        rule: predicate,
                        span: Span {
                            str: "tcp",
                            start: 13,
                            end: 16,
                        },
                        inner: [
                            Pair {
                                rule: protocol,
                                span: Span {
                                    str: "tcp",
                                    start: 13,
                                    end: 16,
                                },
                                inner: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
]
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "ipv4",
        start: 2,
        end: 6,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "ipv4",
                start: 2,
                end: 6,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "ipv4",
                        start: 2,
                        end: 6,
                    },
                    inner: [
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "ipv4",
                                start: 2,
                                end: 6,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "ipv4",
            start: 2,
            end: 6,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "ipv4",
                    start: 2,
                    end: 6,
                },
                inner: [
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "ipv4",
                            start: 2,
                            end: 6,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "ipv4",
        start: 2,
        end: 6,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "ipv4",
                start: 2,
                end: 6,
            },
            inner: [
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "ipv4",
                        start: 2,
                        end: 6,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "ipv4",
            start: 2,
            end: 6,
        },
        inner: [
            Pair {
                rule: protocol,
                span: Span {
                    str: "ipv4",
                    start: 2,
                    end: 6,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "ipv4",
        start: 2,
        end: 6,
    },
    inner: [
        Pair {
            rule: protocol,
            span: Span {
                str: "ipv4",
                start: 2,
                end: 6,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: protocol,
        span: Span {
            str: "ipv4",
            start: 2,
            end: 6,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "ipv4",
        start: 2,
        end: 6,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "ipv4",
        start: 2,
        end: 6,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "ipv4",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
            ),
        ],
    ),
]
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "tcp",
        start: 13,
        end: 16,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "tcp",
                start: 13,
                end: 16,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "tcp",
                        start: 13,
                        end: 16,
                    },
                    inner: [
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "tcp",
                                start: 13,
                                end: 16,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "tcp",
            start: 13,
            end: 16,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "tcp",
                    start: 13,
                    end: 16,
                },
                inner: [
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "tcp",
                            start: 13,
                            end: 16,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "tcp",
        start: 13,
        end: 16,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "tcp",
                start: 13,
                end: 16,
            },
            inner: [
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "tcp",
                        start: 13,
                        end: 16,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "tcp",
            start: 13,
            end: 16,
        },
        inner: [
            Pair {
                rule: protocol,
                span: Span {
                    str: "tcp",
                    start: 13,
                    end: 16,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "tcp",
        start: 13,
        end: 16,
    },
    inner: [
        Pair {
            rule: protocol,
            span: Span {
                str: "tcp",
                start: 13,
                end: 16,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: protocol,
        span: Span {
            str: "tcp",
            start: 13,
            end: 16,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "tcp",
        start: 13,
        end: 16,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "tcp",
        start: 13,
        end: 16,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "tcp",
                    ),
                },
            ),
        ],
    ),
]
parse_conjunct: terms: [
    Disjunct(
        [
            Conjunct(
                [
                    Predicate(
                        Unary {
                            protocol: ProtocolName(
                                "ipv4",
                            ),
                        },
                    ),
                ],
            ),
        ],
    ),
    Disjunct(
        [
            Conjunct(
                [
                    Predicate(
                        Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                    ),
                ],
            ),
        ],
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Disjunct(
                [
                    Conjunct(
                        [
                            Predicate(
                                Unary {
                                    protocol: ProtocolName(
                                        "ipv4",
                                    ),
                                },
                            ),
                        ],
                    ),
                ],
            ),
            Disjunct(
                [
                    Conjunct(
                        [
                            Predicate(
                                Unary {
                                    protocol: ProtocolName(
                                        "tcp",
                                    ),
                                },
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
]
parse_conjunct: terms: [
    Disjunct(
        [
            Conjunct(
                [
                    Disjunct(
                        [
                            Conjunct(
                                [
                                    Predicate(
                                        Unary {
                                            protocol: ProtocolName(
                                                "ipv4",
                                            ),
                                        },
                                    ),
                                ],
                            ),
                        ],
                    ),
                    Disjunct(
                        [
                            Conjunct(
                                [
                                    Predicate(
                                        Unary {
                                            protocol: ProtocolName(
                                                "tcp",
                                            ),
                                        },
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "((ipv6) and (tcp))",
        start: 22,
        end: 40,
    },
    inner: [
        Pair {
            rule: expr,
            span: Span {
                str: "(ipv6) and (tcp)",
                start: 23,
                end: 39,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "(ipv6) and (tcp)",
                        start: 23,
                        end: 39,
                    },
                    inner: [
                        Pair {
                            rule: expr,
                            span: Span {
                                str: "ipv6",
                                start: 24,
                                end: 28,
                            },
                            inner: [
                                Pair {
                                    rule: sub_expr,
                                    span: Span {
                                        str: "ipv6",
                                        start: 24,
                                        end: 28,
                                    },
                                    inner: [
                                        Pair {
                                            rule: predicate,
                                            span: Span {
                                                str: "ipv6",
                                                start: 24,
                                                end: 28,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: protocol,
                                                    span: Span {
                                                        str: "ipv6",
                                                        start: 24,
                                                        end: 28,
                                                    },
                                                    inner: [],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                        Pair {
                            rule: and_op,
                            span: Span {
                                str: "and",
                                start: 30,
                                end: 33,
                            },
                            inner: [],
                        },
                        Pair {
                            rule: expr,
                            span: Span {
                                str: "tcp",
                                start: 35,
                                end: 38,
                            },
                            inner: [
                                Pair {
                                    rule: sub_expr,
                                    span: Span {
                                        str: "tcp",
                                        start: 35,
                                        end: 38,
                                    },
                                    inner: [
                                        Pair {
                                            rule: predicate,
                                            span: Span {
                                                str: "tcp",
                                                start: 35,
                                                end: 38,
                                            },
                                            inner: [
                                                Pair {
                                                    rule: protocol,
                                                    span: Span {
                                                        str: "tcp",
                                                        start: 35,
                                                        end: 38,
                                                    },
                                                    inner: [],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: expr,
        span: Span {
            str: "(ipv6) and (tcp)",
            start: 23,
            end: 39,
        },
        inner: [
            Pair {
                rule: sub_expr,
                span: Span {
                    str: "(ipv6) and (tcp)",
                    start: 23,
                    end: 39,
                },
                inner: [
                    Pair {
                        rule: expr,
                        span: Span {
                            str: "ipv6",
                            start: 24,
                            end: 28,
                        },
                        inner: [
                            Pair {
                                rule: sub_expr,
                                span: Span {
                                    str: "ipv6",
                                    start: 24,
                                    end: 28,
                                },
                                inner: [
                                    Pair {
                                        rule: predicate,
                                        span: Span {
                                            str: "ipv6",
                                            start: 24,
                                            end: 28,
                                        },
                                        inner: [
                                            Pair {
                                                rule: protocol,
                                                span: Span {
                                                    str: "ipv6",
                                                    start: 24,
                                                    end: 28,
                                                },
                                                inner: [],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                    Pair {
                        rule: and_op,
                        span: Span {
                            str: "and",
                            start: 30,
                            end: 33,
                        },
                        inner: [],
                    },
                    Pair {
                        rule: expr,
                        span: Span {
                            str: "tcp",
                            start: 35,
                            end: 38,
                        },
                        inner: [
                            Pair {
                                rule: sub_expr,
                                span: Span {
                                    str: "tcp",
                                    start: 35,
                                    end: 38,
                                },
                                inner: [
                                    Pair {
                                        rule: predicate,
                                        span: Span {
                                            str: "tcp",
                                            start: 35,
                                            end: 38,
                                        },
                                        inner: [
                                            Pair {
                                                rule: protocol,
                                                span: Span {
                                                    str: "tcp",
                                                    start: 35,
                                                    end: 38,
                                                },
                                                inner: [],
                                            },
                                        ],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
]
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "(ipv6) and (tcp)",
        start: 23,
        end: 39,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "(ipv6) and (tcp)",
                start: 23,
                end: 39,
            },
            inner: [
                Pair {
                    rule: expr,
                    span: Span {
                        str: "ipv6",
                        start: 24,
                        end: 28,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "ipv6",
                                start: 24,
                                end: 28,
                            },
                            inner: [
                                Pair {
                                    rule: predicate,
                                    span: Span {
                                        str: "ipv6",
                                        start: 24,
                                        end: 28,
                                    },
                                    inner: [
                                        Pair {
                                            rule: protocol,
                                            span: Span {
                                                str: "ipv6",
                                                start: 24,
                                                end: 28,
                                            },
                                            inner: [],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
                Pair {
                    rule: and_op,
                    span: Span {
                        str: "and",
                        start: 30,
                        end: 33,
                    },
                    inner: [],
                },
                Pair {
                    rule: expr,
                    span: Span {
                        str: "tcp",
                        start: 35,
                        end: 38,
                    },
                    inner: [
                        Pair {
                            rule: sub_expr,
                            span: Span {
                                str: "tcp",
                                start: 35,
                                end: 38,
                            },
                            inner: [
                                Pair {
                                    rule: predicate,
                                    span: Span {
                                        str: "tcp",
                                        start: 35,
                                        end: 38,
                                    },
                                    inner: [
                                        Pair {
                                            rule: protocol,
                                            span: Span {
                                                str: "tcp",
                                                start: 35,
                                                end: 38,
                                            },
                                            inner: [],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "(ipv6) and (tcp)",
            start: 23,
            end: 39,
        },
        inner: [
            Pair {
                rule: expr,
                span: Span {
                    str: "ipv6",
                    start: 24,
                    end: 28,
                },
                inner: [
                    Pair {
                        rule: sub_expr,
                        span: Span {
                            str: "ipv6",
                            start: 24,
                            end: 28,
                        },
                        inner: [
                            Pair {
                                rule: predicate,
                                span: Span {
                                    str: "ipv6",
                                    start: 24,
                                    end: 28,
                                },
                                inner: [
                                    Pair {
                                        rule: protocol,
                                        span: Span {
                                            str: "ipv6",
                                            start: 24,
                                            end: 28,
                                        },
                                        inner: [],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
            Pair {
                rule: and_op,
                span: Span {
                    str: "and",
                    start: 30,
                    end: 33,
                },
                inner: [],
            },
            Pair {
                rule: expr,
                span: Span {
                    str: "tcp",
                    start: 35,
                    end: 38,
                },
                inner: [
                    Pair {
                        rule: sub_expr,
                        span: Span {
                            str: "tcp",
                            start: 35,
                            end: 38,
                        },
                        inner: [
                            Pair {
                                rule: predicate,
                                span: Span {
                                    str: "tcp",
                                    start: 35,
                                    end: 38,
                                },
                                inner: [
                                    Pair {
                                        rule: protocol,
                                        span: Span {
                                            str: "tcp",
                                            start: 35,
                                            end: 38,
                                        },
                                        inner: [],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "(ipv6) and (tcp)",
        start: 23,
        end: 39,
    },
    inner: [
        Pair {
            rule: expr,
            span: Span {
                str: "ipv6",
                start: 24,
                end: 28,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "ipv6",
                        start: 24,
                        end: 28,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "ipv6",
                                start: 24,
                                end: 28,
                            },
                            inner: [
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "ipv6",
                                        start: 24,
                                        end: 28,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
        Pair {
            rule: and_op,
            span: Span {
                str: "and",
                start: 30,
                end: 33,
            },
            inner: [],
        },
        Pair {
            rule: expr,
            span: Span {
                str: "tcp",
                start: 35,
                end: 38,
            },
            inner: [
                Pair {
                    rule: sub_expr,
                    span: Span {
                        str: "tcp",
                        start: 35,
                        end: 38,
                    },
                    inner: [
                        Pair {
                            rule: predicate,
                            span: Span {
                                str: "tcp",
                                start: 35,
                                end: 38,
                            },
                            inner: [
                                Pair {
                                    rule: protocol,
                                    span: Span {
                                        str: "tcp",
                                        start: 35,
                                        end: 38,
                                    },
                                    inner: [],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: expr,
        span: Span {
            str: "ipv6",
            start: 24,
            end: 28,
        },
        inner: [
            Pair {
                rule: sub_expr,
                span: Span {
                    str: "ipv6",
                    start: 24,
                    end: 28,
                },
                inner: [
                    Pair {
                        rule: predicate,
                        span: Span {
                            str: "ipv6",
                            start: 24,
                            end: 28,
                        },
                        inner: [
                            Pair {
                                rule: protocol,
                                span: Span {
                                    str: "ipv6",
                                    start: 24,
                                    end: 28,
                                },
                                inner: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Pair {
        rule: and_op,
        span: Span {
            str: "and",
            start: 30,
            end: 33,
        },
        inner: [],
    },
    Pair {
        rule: expr,
        span: Span {
            str: "tcp",
            start: 35,
            end: 38,
        },
        inner: [
            Pair {
                rule: sub_expr,
                span: Span {
                    str: "tcp",
                    start: 35,
                    end: 38,
                },
                inner: [
                    Pair {
                        rule: predicate,
                        span: Span {
                            str: "tcp",
                            start: 35,
                            end: 38,
                        },
                        inner: [
                            Pair {
                                rule: protocol,
                                span: Span {
                                    str: "tcp",
                                    start: 35,
                                    end: 38,
                                },
                                inner: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
]
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "ipv6",
        start: 24,
        end: 28,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "ipv6",
                start: 24,
                end: 28,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "ipv6",
                        start: 24,
                        end: 28,
                    },
                    inner: [
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "ipv6",
                                start: 24,
                                end: 28,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "ipv6",
            start: 24,
            end: 28,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "ipv6",
                    start: 24,
                    end: 28,
                },
                inner: [
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "ipv6",
                            start: 24,
                            end: 28,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "ipv6",
        start: 24,
        end: 28,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "ipv6",
                start: 24,
                end: 28,
            },
            inner: [
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "ipv6",
                        start: 24,
                        end: 28,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "ipv6",
            start: 24,
            end: 28,
        },
        inner: [
            Pair {
                rule: protocol,
                span: Span {
                    str: "ipv6",
                    start: 24,
                    end: 28,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "ipv6",
        start: 24,
        end: 28,
    },
    inner: [
        Pair {
            rule: protocol,
            span: Span {
                str: "ipv6",
                start: 24,
                end: 28,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: protocol,
        span: Span {
            str: "ipv6",
            start: 24,
            end: 28,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "ipv6",
        start: 24,
        end: 28,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "ipv6",
        start: 24,
        end: 28,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "ipv6",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
            ),
        ],
    ),
]
building from expr: Pair {
    rule: expr,
    span: Span {
        str: "tcp",
        start: 35,
        end: 38,
    },
    inner: [
        Pair {
            rule: sub_expr,
            span: Span {
                str: "tcp",
                start: 35,
                end: 38,
            },
            inner: [
                Pair {
                    rule: predicate,
                    span: Span {
                        str: "tcp",
                        start: 35,
                        end: 38,
                    },
                    inner: [
                        Pair {
                            rule: protocol,
                            span: Span {
                                str: "tcp",
                                start: 35,
                                end: 38,
                            },
                            inner: [],
                        },
                    ],
                },
            ],
        },
    ],
}
parse_disjunct: inner: [
    Pair {
        rule: sub_expr,
        span: Span {
            str: "tcp",
            start: 35,
            end: 38,
        },
        inner: [
            Pair {
                rule: predicate,
                span: Span {
                    str: "tcp",
                    start: 35,
                    end: 38,
                },
                inner: [
                    Pair {
                        rule: protocol,
                        span: Span {
                            str: "tcp",
                            start: 35,
                            end: 38,
                        },
                        inner: [],
                    },
                ],
            },
        ],
    },
]
building from disjunct: Pair {
    rule: sub_expr,
    span: Span {
        str: "tcp",
        start: 35,
        end: 38,
    },
    inner: [
        Pair {
            rule: predicate,
            span: Span {
                str: "tcp",
                start: 35,
                end: 38,
            },
            inner: [
                Pair {
                    rule: protocol,
                    span: Span {
                        str: "tcp",
                        start: 35,
                        end: 38,
                    },
                    inner: [],
                },
            ],
        },
    ],
}
parse_conjunct: inner: [
    Pair {
        rule: predicate,
        span: Span {
            str: "tcp",
            start: 35,
            end: 38,
        },
        inner: [
            Pair {
                rule: protocol,
                span: Span {
                    str: "tcp",
                    start: 35,
                    end: 38,
                },
                inner: [],
            },
        ],
    },
]
parse_predicate: pair: Pair {
    rule: predicate,
    span: Span {
        str: "tcp",
        start: 35,
        end: 38,
    },
    inner: [
        Pair {
            rule: protocol,
            span: Span {
                str: "tcp",
                start: 35,
                end: 38,
            },
            inner: [],
        },
    ],
}
parse_predicate: inner: [
    Pair {
        rule: protocol,
        span: Span {
            str: "tcp",
            start: 35,
            end: 38,
        },
        inner: [],
    },
]
parse_predicate: protocol: Pair {
    rule: protocol,
    span: Span {
        str: "tcp",
        start: 35,
        end: 38,
    },
    inner: [],
}
parse_protocol: pair: Pair {
    rule: protocol,
    span: Span {
        str: "tcp",
        start: 35,
        end: 38,
    },
    inner: [],
}
parse_conjunct: terms: [
    Predicate(
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Predicate(
                Unary {
                    protocol: ProtocolName(
                        "tcp",
                    ),
                },
            ),
        ],
    ),
]
parse_conjunct: terms: [
    Disjunct(
        [
            Conjunct(
                [
                    Predicate(
                        Unary {
                            protocol: ProtocolName(
                                "ipv6",
                            ),
                        },
                    ),
                ],
            ),
        ],
    ),
    Disjunct(
        [
            Conjunct(
                [
                    Predicate(
                        Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                    ),
                ],
            ),
        ],
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Disjunct(
                [
                    Conjunct(
                        [
                            Predicate(
                                Unary {
                                    protocol: ProtocolName(
                                        "ipv6",
                                    ),
                                },
                            ),
                        ],
                    ),
                ],
            ),
            Disjunct(
                [
                    Conjunct(
                        [
                            Predicate(
                                Unary {
                                    protocol: ProtocolName(
                                        "tcp",
                                    ),
                                },
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
]
parse_conjunct: terms: [
    Disjunct(
        [
            Conjunct(
                [
                    Disjunct(
                        [
                            Conjunct(
                                [
                                    Predicate(
                                        Unary {
                                            protocol: ProtocolName(
                                                "ipv6",
                                            ),
                                        },
                                    ),
                                ],
                            ),
                        ],
                    ),
                    Disjunct(
                        [
                            Conjunct(
                                [
                                    Predicate(
                                        Unary {
                                            protocol: ProtocolName(
                                                "tcp",
                                            ),
                                        },
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
]
parse_disjunct: terms: [
    Conjunct(
        [
            Disjunct(
                [
                    Conjunct(
                        [
                            Disjunct(
                                [
                                    Conjunct(
                                        [
                                            Predicate(
                                                Unary {
                                                    protocol: ProtocolName(
                                                        "ipv4",
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                            Disjunct(
                                [
                                    Conjunct(
                                        [
                                            Predicate(
                                                Unary {
                                                    protocol: ProtocolName(
                                                        "tcp",
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
    Conjunct(
        [
            Disjunct(
                [
                    Conjunct(
                        [
                            Disjunct(
                                [
                                    Conjunct(
                                        [
                                            Predicate(
                                                Unary {
                                                    protocol: ProtocolName(
                                                        "ipv6",
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                            Disjunct(
                                [
                                    Conjunct(
                                        [
                                            Predicate(
                                                Unary {
                                                    protocol: ProtocolName(
                                                        "tcp",
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
]
ast: Disjunct(
    [
        Conjunct(
            [
                Disjunct(
                    [
                        Conjunct(
                            [
                                Disjunct(
                                    [
                                        Conjunct(
                                            [
                                                Predicate(
                                                    Unary {
                                                        protocol: ProtocolName(
                                                            "ipv4",
                                                        ),
                                                    },
                                                ),
                                            ],
                                        ),
                                    ],
                                ),
                                Disjunct(
                                    [
                                        Conjunct(
                                            [
                                                Predicate(
                                                    Unary {
                                                        protocol: ProtocolName(
                                                            "tcp",
                                                        ),
                                                    },
                                                ),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        ),
        Conjunct(
            [
                Disjunct(
                    [
                        Conjunct(
                            [
                                Disjunct(
                                    [
                                        Conjunct(
                                            [
                                                Predicate(
                                                    Unary {
                                                        protocol: ProtocolName(
                                                            "ipv6",
                                                        ),
                                                    },
                                                ),
                                            ],
                                        ),
                                    ],
                                ),
                                Disjunct(
                                    [
                                        Conjunct(
                                            [
                                                Predicate(
                                                    Unary {
                                                        protocol: ProtocolName(
                                                            "tcp",
                                                        ),
                                                    },
                                                ),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        ),
    ],
)
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "ipv4",
            ),
        },
    ],
]
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ],
]
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "ipv4",
            ),
        },
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ],
]
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "ipv6",
            ),
        },
    ],
]
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ],
]
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "ipv6",
            ),
        },
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ],
]
flat_conjuncts: [
    [
        Unary {
            protocol: ProtocolName(
                "ipv4",
            ),
        },
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ],
    [
        Unary {
            protocol: ProtocolName(
                "ipv6",
            ),
        },
        Unary {
            protocol: ProtocolName(
                "tcp",
            ),
        },
    ],
]
ptree: FlatPTree {
    root: FlatPNode {
        id: 0,
        pred: Unary {
            protocol: ProtocolName(
                "ethernet",
            ),
        },
        is_terminal: false,
        terminates: None,
        patterns: [
            0,
            1,
        ],
        children: [
            FlatPNode {
                id: 1,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv4",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    0,
                ],
                children: [
                    FlatPNode {
                        id: 2,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: true,
                        terminates: Packet,
                        patterns: [
                            0,
                        ],
                        children: [],
                    },
                ],
            },
            FlatPNode {
                id: 3,
                pred: Unary {
                    protocol: ProtocolName(
                        "ipv6",
                    ),
                },
                is_terminal: false,
                terminates: None,
                patterns: [
                    1,
                ],
                children: [
                    FlatPNode {
                        id: 4,
                        pred: Unary {
                            protocol: ProtocolName(
                                "tcp",
                            ),
                        },
                        is_terminal: true,
                        terminates: Packet,
                        patterns: [
                            1,
                        ],
                        children: [],
                    },
                ],
            },
        ],
    },
    size: 5,
}
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use retina_core::{config::load_config, Runtime};
use retina_datatypes::{DnsTransaction, HttpTransaction, TlsHandshake};
use retina_filtergen::{filter, retina_main};
use clap::Parser;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct file {
    __private_field: (),
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
static file: file = file { __private_field: () };
impl ::lazy_static::__Deref for file {
    type Target = Mutex<BufWriter<File>>;
    fn deref(&self) -> &Mutex<BufWriter<File>> {
        #[inline(always)]
        fn __static_ref_initialize() -> Mutex<BufWriter<File>> {
            Mutex::new(BufWriter::new(File::create("test_not_protocol.jsonl").unwrap()))
        }
        #[inline(always)]
        fn __stability() -> &'static Mutex<BufWriter<File>> {
            static LAZY: ::lazy_static::lazy::Lazy<Mutex<BufWriter<File>>> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for file {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
struct Args {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: PathBuf,
    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "FILE",
        default_value = "test_not_protocol.jsonl"
    )]
    outfile: PathBuf,
}
impl clap::Parser for Args {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[allow(deprecated)]
impl clap::CommandFactory for Args {
    fn into_app<'b>() -> clap::Command<'b> {
        let __clap_app = clap::Command::new("test_not_protocol");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn into_app_for_update<'b>() -> clap::Command<'b> {
        let __clap_app = clap::Command::new("test_not_protocol");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
impl clap::FromArgMatches for Args {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Args {
            config: __clap_arg_matches
                .get_one::<::std::ffi::OsString>("config")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "config",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?,
            outfile: __clap_arg_matches
                .get_one::<::std::ffi::OsString>("outfile")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "outfile",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("config") {
            #[allow(non_snake_case)]
            let config = &mut self.config;
            *config = __clap_arg_matches
                .get_one::<::std::ffi::OsString>("config")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "config",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?;
        }
        if __clap_arg_matches.contains_id("outfile") {
            #[allow(non_snake_case)]
            let outfile = &mut self.outfile;
            *outfile = __clap_arg_matches
                .get_one::<::std::ffi::OsString>("outfile")
                .map(|s| ::std::ops::Deref::deref(s))
                .ok_or_else(|| clap::Error::raw(
                    clap::ErrorKind::MissingRequiredArgument,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The following required argument was not provided: {0}",
                                "outfile",
                            ),
                        );
                        res
                    }),
                ))
                .and_then(|s| ::std::result::Result::Ok::<
                    _,
                    clap::Error,
                >(::std::convert::From::from(s)))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
impl clap::Args for Args {
    fn augment_args<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
        {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("config")
                        .takes_value(true)
                        .value_name("CONFIG")
                        .required(true && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg.short('c').long("config").value_name("FILE");
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("outfile")
                        .takes_value(true)
                        .value_name("OUTFILE")
                        .required(false && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg
                        .short('o')
                        .long("outfile")
                        .value_name("FILE")
                        .default_value("test_not_protocol.jsonl");
                    arg
                });
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
        {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("config")
                        .takes_value(true)
                        .value_name("CONFIG")
                        .required(false && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg.short('c').long("config").value_name("FILE");
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("outfile")
                        .takes_value(true)
                        .value_name("OUTFILE")
                        .required(false && clap::ArgAction::StoreValue.takes_values())
                        .value_parser(clap::builder::ValueParser::os_string())
                        .action(clap::ArgAction::StoreValue);
                    let arg = arg
                        .short('o')
                        .long("outfile")
                        .value_name("FILE")
                        .default_value("test_not_protocol.jsonl");
                    arg
                });
            __clap_app
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Args {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Args",
            "config",
            &self.config,
            "outfile",
            &&self.outfile,
        )
    }
}
fn not_http_cb2(http: &HttpTransaction) {
    {
        ::std::io::_print(format_args!("cb2\n"));
    };
    if let Ok(serialized) = serde_json::to_string(&http) {
        let mut wtr = file.lock().unwrap();
        wtr.write_all(serialized.as_bytes()).unwrap();
        wtr.write_all(b"\n").unwrap();
    }
}
use retina_core::filter::actions::*;
use retina_core::subscription::{Trackable, Subscribable};
use retina_datatypes::{FromSession, Tracked, FromMbuf, StaticData, PacketList};
pub struct SubscribedWrapper;
impl Subscribable for SubscribedWrapper {
    type Tracked = TrackedWrapper;
}
pub struct TrackedWrapper {
    sessions: Vec<retina_core::protocols::Session>,
    mbufs: Vec<retina_core::Mbuf>,
    core_id: retina_core::CoreId,
}
impl Trackable for TrackedWrapper {
    type Subscribed = SubscribedWrapper;
    fn new(pdu: &retina_core::L4Pdu, core_id: retina_core::CoreId) -> Self {
        Self {
            sessions: ::alloc::vec::Vec::new(),
            mbufs: ::alloc::vec::Vec::new(),
            core_id,
        }
    }
    fn update(&mut self, pdu: &retina_core::L4Pdu, reassembled: bool) {}
    fn core_id(&self) -> &retina_core::CoreId {
        &self.core_id
    }
    fn buffer_packet(
        &mut self,
        pdu: &retina_core::L4Pdu,
        actions: &Actions,
        reassembled: bool,
    ) {
        if !reassembled && actions.data.intersects(ActionData::PacketCache) {
            self.mbufs.push(retina_core::Mbuf::new_ref(&pdu.mbuf));
        }
        if actions.data.intersects(ActionData::PacketTrack) {}
    }
    fn packets(&self) -> &Vec<retina_core::Mbuf> {
        &self.mbufs
    }
    fn drain_cached_packets(&mut self) {
        self.mbufs = ::alloc::vec::Vec::new();
    }
    fn drain_tracked_packets(&mut self) {}
    fn clear(&mut self) {
        self.drain_tracked_packets();
        self.drain_cached_packets();
        self.sessions = ::alloc::vec::Vec::new();
    }
    fn sessions(&self) -> &Vec<retina_core::protocols::Session> {
        &self.sessions
    }
    fn track_session(&mut self, session: retina_core::protocols::Session) {
        self.sessions.push(session);
    }
    fn parsers() -> retina_core::protocols::stream::ParserRegistry {
        retina_core::protocols::stream::ParserRegistry::from_strings(
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new(["http"])),
        )
    }
}
pub fn filter() -> retina_core::filter::FilterFactory<TrackedWrapper> {
    fn packet_continue(
        mbuf: &retina_core::Mbuf,
        core_id: &retina_core::CoreId,
    ) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if let Ok(ethernet) = &retina_core::protocols::packet::Packet::parse_to::<
            retina_core::protocols::packet::ethernet::Ethernet,
        >(mbuf) {
            if let Ok(ipv4) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv4::Ipv4,
            >(ethernet) {
                if let Ok(tcp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::tcp::Tcp,
                >(ipv4) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(1),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                }
            } else if let Ok(ipv6) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv6::Ipv6,
            >(ethernet) {
                if let Ok(tcp) = &retina_core::protocols::packet::Packet::parse_to::<
                    retina_core::protocols::packet::tcp::Tcp,
                >(ipv6) {
                    result
                        .push(
                            &Actions {
                                data: ActionData::from(1),
                                terminal_actions: ActionData::from(0),
                            },
                        );
                }
            }
        }
        result
    }
    fn packet_filter(mbuf: &retina_core::Mbuf, tracked: &TrackedWrapper) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if let Ok(ethernet) = &retina_core::protocols::packet::Packet::parse_to::<
            retina_core::protocols::packet::ethernet::Ethernet,
        >(mbuf) {
            if let Ok(ipv4) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv4::Ipv4,
            >(ethernet) {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(32),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            } else if let Ok(ipv6) = &retina_core::protocols::packet::Packet::parse_to::<
                retina_core::protocols::packet::ipv6::Ipv6,
            >(ethernet) {
                result
                    .push(
                        &Actions {
                            data: ActionData::from(32),
                            terminal_actions: ActionData::from(0),
                        },
                    );
            }
        }
        result
    }
    fn protocol_filter(
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if !match conn.service() {
            retina_core::protocols::stream::ConnParser::Http { .. } => true,
            _ => false,
        } {
            result
                .push(
                    &Actions {
                        data: ActionData::from(128),
                        terminal_actions: ActionData::from(0),
                    },
                );
        }
        result
    }
    fn session_filter(
        session: &retina_core::protocols::Session,
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) -> Actions {
        let mut result = retina_core::filter::Actions::new();
        if !match &session.data {
            retina_core::protocols::stream::SessionData::Http(http) => true,
            _ => false,
        } {
            if let Some(s) = HttpTransaction::from_session(session) {
                not_http_cb2(s);
            }
        }
        result
    }
    fn packet_deliver(
        mbuf: &retina_core::Mbuf,
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) {}
    fn connection_deliver(
        conn: &retina_core::protocols::ConnData,
        tracked: &TrackedWrapper,
    ) {}
    retina_core::filter::FilterFactory::new(
        "((ipv4) and (tcp)) or ((ipv6) and (tcp))",
        packet_continue,
        packet_filter,
        protocol_filter,
        session_filter,
        packet_deliver,
        connection_deliver,
    )
}
fn main() {
    let args = Args::parse();
    let config = load_config(&args.config);
    let mut runtime: Runtime<SubscribedWrapper> = Runtime::new(config, filter).unwrap();
    runtime.run();
    let mut wtr = file.lock().unwrap();
    wtr.flush().unwrap();
}
