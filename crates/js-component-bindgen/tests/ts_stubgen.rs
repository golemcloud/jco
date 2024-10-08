use std::collections::HashMap;

use js_component_bindgen::generate_typescript_stubs;

// Enable this to write the generated files to the `tests/temp` directory
static IS_DEBUG: bool = false;

#[test]
fn basic_ts() {
    let wit = "
        package test:t-basic;

        world test {
            export basic-test;
        }

        interface basic-test {
            bool-test: func(a: bool) -> bool;
            s8-test: func(a: s8) -> s8;
            s16-test: func(a: s16) -> s16;
            s32-test: func(a: s32) -> s32;
            s64-test: func(a: s64) -> s64;
            u8-test: func(a: u8) -> u8;
            u16-test: func(a: u16) -> u16;
            u32-test: func(a: u32) -> u32;
            u64-test: func(a: u64) -> u64;
            f32-test: func(a: f32) -> f32;
            f64-test: func(a: f64) -> f64;
            char-test: func(c: char) -> char;
            string-test: func(s: string) -> string;
            u8-list-test: func(a: list<u8>) -> list<u8>;
            string-list-test: func(a: list<string>) -> list<string>;
            option-nullable-test: func(a: option<u8>) -> option<u8>;
            option-nested-test: func(a: option<option<u8>>) -> option<option<u8>>;
            result-success-test: func(a: u8) -> result<u8>;
            result-fail-test: func(a: u8) -> result<_, u8>;
            result-both-test: func(a: u8) -> result<u8, u8>;
            result-none-test: func() -> result;

            variant variant-test {
                none,
                any,
                something(string)
            }

            variant enum-test {
                a,
                b,
                c,
            }

            record record-test {
                a: u8,
                b: string,
                c: option<enum-test>
            }

            flags flag-test {
                a,
                b,
                c,
            }
        }
    ";

    let expected = "
        export interface BasicTest {
            boolTest(a: boolean): boolean,
            s8Test(a: number): number,
            s16Test(a: number): number,
            s32Test(a: number): number,
            s64Test(a: bigint): bigint,
            u8Test(a: number): number,
            u16Test(a: number): number,
            u32Test(a: number): number,
            u64Test(a: bigint): bigint,
            f32Test(a: number): number,
            f64Test(a: number): number,
            charTest(c: string): string,
            stringTest(s: string): string,
            u8ListTest(a: Uint8Array): Uint8Array,
            stringListTest(a: string[]): string[],
            optionNullableTest(a: number | undefined): number | undefined,
            optionNestedTest(a: Option<number | undefined>): Option<number | undefined>,
            resultSuccessTest(a: number): Result<number, void>,
            resultFailTest(a: number): Result<void, number>,
            resultBothTest(a: number): Result<number, number>,
            resultNoneTest(): Result<void, void>,
        }
        export type VariantTest = VariantTestNone | VariantTestAny | VariantTestSomething;
        export interface VariantTestNone {
            tag: 'none',
        }
        export interface VariantTestAny {
            tag: 'any',
        }
        export interface VariantTestSomething {
            tag: 'something',
            val: string,
        }
        export type EnumTest = EnumTestA | EnumTestB | EnumTestC;
        export interface EnumTestA {
            tag: 'a',
        }
        export interface EnumTestB {
            tag: 'b',
        }
        export interface EnumTestC {
            tag: 'c',
        }
        export interface RecordTest {
            a: number,
            b: string,
            c?: EnumTest,
        }
        export interface FlagTest {
            a?: boolean,
            b?: boolean,
            c?: boolean,
        }
        export type Option<T> = { tag: 'none' } | { tag: 'some', val: T };
        export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };

        export interface TestWorld {
            basicTest: BasicTest,
        }
";

    test_single_file(wit, expected);
}

#[test]
fn export_resource() {
    let wit = "
        package test:t-resource;

        world test {
            export resource-test;
        }

        interface resource-test {
            resource blob {
                constructor(init: list<u8>);
                write: func(bytes: list<u8>);
                read: func(n: u32) -> list<u8>;
                merge: static func(lhs: blob, rhs: blob) -> blob;
                some-optional: func(a: option<u8>) -> option<u8>;
            }
        }
    ";

    let expected = "
        export interface ResourceTest {
            Blob: BlobStatic
        }

        export interface BlobStatic {
            new(init: Uint8Array): BlobInstance,
            merge(lhs: BlobInstance, rhs: BlobInstance): BlobInstance,
        }
        export interface BlobInstance {
            write(bytes: Uint8Array): void,
            read(n: number): Uint8Array,
            someOptional(a: number | undefined): number | undefined,
        }

        export interface TestWorld {
            resourceTest: ResourceTest,
        }
    ";

    test_single_file(wit, expected);
}

#[test]
fn resource_import() {
    let wit = &[
        WitFile {
            wit: "
            package test:resource-source;

            interface resource-test {
                resource blob {
                    constructor(init: list<u8>);
                    write: func(bytes: list<u8>);
                    read: func(n: u32) -> list<u8>;
                    merge: static func(lhs: blob, rhs: blob) -> blob;
                }
            }
            ",
        },
        WitFile {
            wit: "
            package test:resource-import;

             world test {
                import test:resource-source/resource-test;
                export run: func() -> ();
            }
            ",
        },
    ];

    let expected = &[
        ExpectedTs {
            file_name: "test.d.ts",
            expected: "
            export interface TestWorld {
                run(): void,
            }",
        },
        ExpectedTs {
            file_name: "interfaces/test-resource-source-resource-test.d.ts",
            expected: r#"
            declare module "test:resource-source/resource-test" {
                export class Blob {
                    constructor(init: Uint8Array)
                    write(bytes: Uint8Array): void;
                    read(n: number): Uint8Array;
                    static merge(lhs: Blob, rhs: Blob): Blob;
                }
            }
            "#,
        },
    ];

    test_files(wit, expected);
}

#[test]
fn imports() {
    let wit = &[
        WitFile {
            wit: "
                package test:types;

                interface types {
                    type dimension = u32;
                    record point {
                        x: dimension,
                        y: dimension,
                    }
                }
            ",
        },
        WitFile {
            wit: "
                package test:canvas;

                interface canvas {
                    use test:types/types.{dimension, point};
                    type canvas-id = u64;
                    draw-line: func(canvas: canvas-id, origin: point, target: point, thickness: dimension);
                }
            ",
        },
        WitFile {
            wit: "
                package test:t-imports;

                world test {
                    import test:canvas/canvas;
                    export run: func();
                }
            ",
        },
    ];

    let expected = &[
        ExpectedTs {
            file_name: "test.d.ts",
            expected: "
            export interface TestWorld {
                run(): void,
            }",
        },
        ExpectedTs {
            file_name: "interfaces/test-canvas-canvas.d.ts",
            expected: r#"
            declare module "test:canvas/canvas" {
                import type { Dimension } from "test:types/types";
                import type { Point } from "test:types/types";
                export type CanvasId = bigint;
                export function drawLine(canvas: CanvasId, origin: Point, target: Point, thickness: Dimension): void;
            } 
            "#,
        },
        ExpectedTs {
            file_name: "interfaces/test-types-types.d.ts",
            expected: r#"
            declare module "test:types/types" {
                export type Dimension = number;
                export interface Point {
                    x: Dimension,
                    y: Dimension,
                }
            } 
            "#,
        },
    ];

    test_files(wit, expected);
}

#[test]
fn rpc() {
    let wit = &[
        WitFile {
            wit: "
            package golem:rpc@0.1.0;

            interface types {
                type node-index = s32;

                record wit-value {
                    nodes: list<wit-node>,
                }

                variant wit-node {
                    record-value(list<node-index>),
                    variant-value(tuple<u32, option<node-index>>),
                    enum-value(u32),
                    flags-value(list<bool>),
                    tuple-value(list<node-index>),
                    list-value(list<node-index>),
                    option-value(option<node-index>),
                    result-value(result<option<node-index>, option<node-index>>),
                    prim-u8(u8),
                    prim-u16(u16),
                    prim-u32(u32),
                    prim-u64(u64),
                    prim-s8(s8),
                    prim-s16(s16),
                    prim-s32(s32),
                    prim-s64(s64),
                    prim-float32(float32),
                    prim-float64(float64),
                    prim-char(char),
                    prim-bool(bool),
                    prim-string(string),
                    handle(tuple<uri, u64>)
                }

                record uri {
                    value: string,
                }

                variant rpc-error {
                    protocol-error(string),
                    denied(string),
                    not-found(string),
                    remote-internal-error(string)
                }

                resource wasm-rpc {
                    constructor(location: uri);

                    invoke-and-await: func(function-name: string, function-params: list<wit-value>) -> result<wit-value, rpc-error>;
                    invoke: func(function-name: string, function-params: list<wit-value>) -> result<_, rpc-error>;
                }
            }

            world wit-value {
                import types;
            }
            ",
        },
        WitFile {
            wit: "
            package rpc:counters;

            interface api {

                record counter-value {
                    value: u64,
                }

                resource counter {
                    constructor(name: string);
                    inc-by: func(value: u64);
                    get-value: func() -> counter-value;
                }

                inc-global-by: func(value: u64);
                get-global-value: func() -> counter-value;
                get-all-dropped: func() -> list<tuple<string, u64>>;
            }

            world counters {
                export api;
            }
            ",
        },
        WitFile {
            wit: "
            package rpc:counters-stub;

            interface stub-counters {
                use golem:rpc/types@0.1.0.{uri};
                use rpc:counters/api.{counter-value};

                resource api {
                    constructor(location: uri);
                    inc-global-by: func(value: u64);
                    get-global-value: func() -> counter-value;
                    get-all-dropped: func() -> list<tuple<string, u64>>;
                }

                resource counter {
                    constructor(location: uri, name: string);
                    inc-by: func(value: u64);
                    get-value: func() -> counter-value;
                }

            }

            world wasm-rpc-stub-counters {
                export stub-counters;
            }
            ",
        },
        WitFile {
            wit: "
            package test:rpc;

            interface api {
                test1: func() -> list<tuple<string, u64>>;
                test2: func() -> u64;
                test3: func() -> u64;
            }

            world test {
                import rpc:counters-stub/stub-counters;
                export api;
            }
            "
        }
    ];

    let expected = &[
        ExpectedTs {
            file_name: "test.d.ts",
            expected: "
            export interface Api {
                test1(): [string, bigint][],
                test2(): bigint,
                test3(): bigint,
            }

            export interface TestWorld {
                api: Api,
            }
            ",
        },
        ExpectedTs {
            file_name: "interfaces/golem-rpc-types.d.ts",
            expected: r#"
            declare module "golem:rpc/types@0.1.0" {
                export type NodeIndex = number;
                export interface Uri {
                    value: string,
                }
                export type WitNode = WitNodeRecordValue | WitNodeVariantValue | WitNodeEnumValue | WitNodeFlagsValue | WitNodeTupleValue | WitNodeListValue | WitNodeOptionValue | WitNodeResultValue | WitNodePrimU8 | WitNodePrimU16 | WitNodePrimU32 | WitNodePrimU64 | WitNodePrimS8 | WitNodePrimS16 | WitNodePrimS32 | WitNodePrimS64 | WitNodePrimFloat32 | WitNodePrimFloat64 | WitNodePrimChar | WitNodePrimBool | WitNodePrimString | WitNodeHandle;
                export interface WitNodeRecordValue {
                    tag: 'record-value',
                    val: Int32Array,
                }
                export interface WitNodeVariantValue {
                    tag: 'variant-value',
                    val: [number, NodeIndex | undefined],
                }
                export interface WitNodeEnumValue {
                    tag: 'enum-value',
                    val: number,
                }
                export interface WitNodeFlagsValue {
                    tag: 'flags-value',
                    val: boolean[],
                }
                export interface WitNodeTupleValue {
                    tag: 'tuple-value',
                    val: Int32Array,
                }
                export interface WitNodeListValue {
                    tag: 'list-value',
                    val: Int32Array,
                }
                export interface WitNodeOptionValue {
                    tag: 'option-value',
                    val: NodeIndex | undefined,
                }
                export interface WitNodeResultValue {
                    tag: 'result-value',
                    val: Result<NodeIndex | undefined, NodeIndex | undefined>,
                }
                export interface WitNodePrimU8 {
                    tag: 'prim-u8',
                    val: number,
                }
                export interface WitNodePrimU16 {
                    tag: 'prim-u16',
                    val: number,
                }
                export interface WitNodePrimU32 {
                    tag: 'prim-u32',
                    val: number,
                }
                export interface WitNodePrimU64 {
                    tag: 'prim-u64',
                    val: bigint,
                }
                export interface WitNodePrimS8 {
                    tag: 'prim-s8',
                    val: number,
                }
                export interface WitNodePrimS16 {
                    tag: 'prim-s16',
                    val: number,
                }
                export interface WitNodePrimS32 {
                    tag: 'prim-s32',
                    val: number,
                }
                export interface WitNodePrimS64 {
                    tag: 'prim-s64',
                    val: bigint,
                }
                export interface WitNodePrimFloat32 {
                    tag: 'prim-float32',
                    val: number,
                }
                export interface WitNodePrimFloat64 {
                    tag: 'prim-float64',
                    val: number,
                }
                export interface WitNodePrimChar {
                    tag: 'prim-char',
                    val: string,
                }
                export interface WitNodePrimBool {
                    tag: 'prim-bool',
                    val: boolean,
                }
                export interface WitNodePrimString {
                    tag: 'prim-string',
                    val: string,
                }
                export interface WitNodeHandle {
                    tag: 'handle',
                    val: [Uri, bigint],
                }
                export interface WitValue {
                    nodes: WitNode[],
                }
                export type RpcError = RpcErrorProtocolError | RpcErrorDenied | RpcErrorNotFound | RpcErrorRemoteInternalError;
                export interface RpcErrorProtocolError {
                    tag: 'protocol-error',
                    val: string,
                }
                export interface RpcErrorDenied {
                    tag: 'denied',
                    val: string,
                }
                export interface RpcErrorNotFound {
                    tag: 'not-found',
                    val: string,
                }
                export interface RpcErrorRemoteInternalError {
                    tag: 'remote-internal-error',
                    val: string,
                }
                
                export class WasmRpc {
                    constructor(location: Uri)
                    invokeAndAwait(functionName: string, functionParams: WitValue[]): Result<WitValue, RpcError>;
                    invoke(functionName: string, functionParams: WitValue[]): Result<void, RpcError>;
                }

                export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };
            }
            "#,
        },
        ExpectedTs {
            file_name: "interfaces/rpc-counters-stub-stub-counters.d.ts",
            expected: r#"
            declare module "rpc:counters-stub/stub-counters" {
                import type { Uri } from "golem:rpc/types@0.1.0";
                import type { CounterValue } from "rpc:counters/api";
                
                export class Api {
                    constructor(location: Uri)
                    incGlobalBy(value: bigint): void;
                    getGlobalValue(): CounterValue;
                    getAllDropped(): [string, bigint][];
                }
                
                export class Counter {
                    constructor(location: Uri, name: string)
                    incBy(value: bigint): void;
                    getValue(): CounterValue;
                }
            }
            "#,
        },
        ExpectedTs {
            file_name: "interfaces/rpc-counters-api.d.ts",
            expected: r#"
            declare module "rpc:counters/api" {
                export interface CounterValue {
                    value: bigint,
                }
                export function incGlobalBy(value: bigint): void;
                export function getGlobalValue(): CounterValue;
                export function getAllDropped(): [string, bigint][];
                
                export class Counter {
                    constructor(name: string)
                    incBy(value: bigint): void;
                    getValue(): CounterValue;
                }
            }
            "#,
        },
    ];

    test_files(wit, expected);
}

#[test]
fn inline_interface() {
    let wit = "

    package test:inline;
    
    world test {
        export example: interface {
            do-nothing: func();
        }
    }
    ";

    let expected = "
    export interface Example {
        doNothing(): void,
    }

    export interface TestWorld {
        example: Example,
    }
    ";

    test_single_file(wit, expected);
}

#[test]
fn reserved_keywords() {
    let wit = &[
        WitFile {
            wit: "
                package test:kv;

                interface kv {
                    type key = string;
                    type value = string;

                    get: func(key: key) -> value;
                    set: func(key: key, value: value);
                    delete: func(key: key);
                }
            ",
        },
        WitFile {
            wit: "
                package test:reserved;

                world test {
                    import test:kv/kv;

                    export run: func();
                }
            ",
        },
    ];

    let expected = &[
        ExpectedTs {
            file_name: "test.d.ts",
            expected: "
            export interface TestWorld {
                run(): void,
            }",
        },
        ExpectedTs {
            file_name: "interfaces/test-kv-kv.d.ts",
            expected: r#"
            declare module "test:kv/kv" {
                export type Key = string;
                export type Value = string;
                export function get(key: Key): Value;
                export function set(key: Key, value: Value): void;
                export { _delete as delete };
                declare function _delete(key: Key): void;
            }
            "#,
        },
    ];

    test_files(wit, expected);
}

struct WitFile {
    wit: &'static str,
}

struct ExpectedTs {
    file_name: &'static str,
    expected: &'static str,
}

#[track_caller]
fn test_files(wit: &[WitFile], expected: &[ExpectedTs]) {
    let mut resolver = js_component_bindgen::source::wit_parser::Resolve::default();

    for (ii, wit_file) in wit.iter().enumerate() {
        let file_name = format!("tests{ii}.wit");
        let package =
            wit_parser::UnresolvedPackageGroup::parse(&file_name, wit_file.wit).expect("valid wit");
        resolver.push_group(package).expect("push package");
    }

    test_resolved_world(resolver, expected);
}

#[track_caller]
fn test_resolved_world(
    resolver: js_component_bindgen::source::wit_parser::Resolve,
    expected: &[ExpectedTs],
) {
    let world = resolver
        .worlds
        .iter()
        .find(|(_, w)| w.name == "test")
        .expect("world exists")
        .0;

    let mut files = generate_typescript_stubs(resolver, world)
        .expect("Successful generation")
        .into_iter()
        .collect::<HashMap<_, _>>();

    if IS_DEBUG {
        write_files(files.iter());
    }

    for ExpectedTs {
        file_name,
        expected,
    } in expected
    {
        let Some(file) = files.remove(*file_name) else {
            let all_files = files.iter().map(|(name, _)| name).collect::<Vec<_>>();
            panic!("Expected file `{file_name}` not found in files: {all_files:?}",)
        };
        let actual = std::str::from_utf8(&file).expect("valid utf8");
        compare_str(actual, expected);
    }

    if !files.is_empty() {
        let all_files = files.iter().map(|(name, _)| name).collect::<Vec<_>>();
        panic!("Missing expected files: {all_files:?}")
    }
}

#[track_caller]
fn test_single_file(wit: &'static str, expected: &'static str) {
    test_files(
        &[WitFile { wit }],
        &[ExpectedTs {
            file_name: "test.d.ts",
            expected,
        }],
    )
}

#[track_caller]
fn compare_str(actual: &str, expected: &str) {
    fn remove_whitespace(s: &str) -> impl Iterator<Item = &str> {
        s.lines().map(|l| l.trim()).filter(|l| !l.is_empty())
    }

    let mut expected_iter = remove_whitespace(expected);
    let mut actual_iter = remove_whitespace(actual);

    loop {
        match (expected_iter.next(), actual_iter.next()) {
            (None, None) => break,
            (Some(e), Some(a)) => {
                assert_eq!(e, a, "\nExpected:`{e}`\nActual:`{a}`\nFull:\n{actual}");
            }
            (e, a) => {
                assert_eq!(e, a, "\nExpected:`{e:?}`\nActual:`{a:?}`\nFull:\n{actual}");
            }
        }
    }
}

fn write_files<'a>(files: impl Iterator<Item = (&'a String, &'a Vec<u8>)>) {
    let prefix = std::path::Path::new("tests/temp");
    let _ = std::fs::remove_dir_all(&prefix);
    for (name, data) in files {
        let name = name.to_string();
        let data = String::from_utf8(data.to_vec()).unwrap();
        let path = prefix.join(name);

        // Create the parent directory if it doesn't exist
        std::fs::create_dir_all(path.parent().unwrap()).expect("Create parent directory");

        std::fs::write(path, data).expect("Write file");
    }
}
