import { shallowMount } from "@vue/test-utils";
import { name } from "./{name}.vue";

describe("HelloWorld", () => {
  let component;
  beforeEach(() => {
    component = shallowMount({ name });
  });
});
