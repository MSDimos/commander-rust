import React from 'react';
import ProgressiveCodeBlock from '../components/progressiveCodeBlock';
import './styles/shades.css';

export default function Shades(props = {
  expand: false,
  children: [],
  code: '',
  content: '',
}) {

  const { expand, code, content } = props;
  const upClass = expand ? 'shade hide' : 'shade';
  const cbClass = expand ? 'code-block hide' : 'code-block';
  const contentClass = expand ? 'content show' : 'content';

  return (
    <React.Fragment>
      <div className='shades'>
        <div className={ upClass }></div>

        <div className={ cbClass }>
          <ProgressiveCodeBlock code={code} />
        </div>
        <div dangerouslySetInnerHTML={{
          __html: content,
        }} className={ contentClass }></div>

        <div className={ upClass }></div>
      </div>
    </React.Fragment>
  );
}
