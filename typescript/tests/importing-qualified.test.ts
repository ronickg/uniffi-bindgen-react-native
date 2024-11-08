/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
 */

import * as exported from "./playground/exported";

import { test } from "../testing/asserts";

test("Records imported as type", (t) => {
  const record: exported.MyRecord = {
    prop1: "string",
    prop2: 42,
  };
});

test("Enums imported as objects", (t) => {
  const enum_: exported.MyEnum = new exported.MyEnum.Variant1();
});

test("Objects interfaces imported as types", (t) => {
  // In our generated code, `MyObject` would not be imported into this
  // file because all creation happens via the `FfiConverterTypeMyObject`.
  const obj: exported.MyObjectInterface = new exported.MyObject();
});

test("Callback interfaces imported as types", (t) => {
  class Impl implements exported.MyCallbackInterface {
    myMethod(): void {}
  }

  const cb = new Impl();
});

test("Custom types imported as types", (t) => {
  const s: exported.MyCustomString = "string";
});
