pub const COMPONENT_TEMPLATE: &'static str = "\
import React from 'react';
import PropTypes from 'prop-types';
import bemCl from 'bem-cl';
import './{{file_name}}.scss';

const b = bemCl('{{style_name}}');

class {{component_name}} extends React.PureComponent {
    render() {
        return (
            <div>
                Hello world
            </div>
        );
    }
}

export default {{component_name}};
";

pub const STYLE_TEMPLATE: &'static str = "\
.{{style_name}} {

}
";

pub const INDEX_TEMPLATE: &'static str = "\
export default from './{{file_name}}.jsx';
";
