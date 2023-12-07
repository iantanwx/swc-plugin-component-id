const {
  transform,
} = require("../dist/nodejs/swc_plugin_react_component_id.js");

describe("Test", () => {
  it("should just work", () => {
    const code = `
			import React from 'react';
			import { Button } from 'antd';

			export default () => (
				<div>
					<Button type="primary">PRESS ME</Button>
				</div>
			);
		`;

    const result = transform(code);

    console.log("result: ", result.code);
  });
});
