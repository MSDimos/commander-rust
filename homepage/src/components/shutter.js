import React from 'react';
import ProgressiveCodeBlock from '../components/progressiveCodeBlock';

import './styles/shutter.css';

export default function Shutter(props = {
  expand: false,
  children: [],
  code: ''
}) {

  const { children, expand, code } = props;
  const expandClass = expand ? 'wrap expand' : 'wrap';
  const upflowClass = expand ? 'content upflow' :'content';
  const insideClass = expand ? 'inside show' : 'inside';

  return (
    <div className={expandClass}>
      <div className='shutter-wrap'>
        <div className='up shutter'>
          <div className='header'>
            <h4>commander.rust</h4>
            <div>
              <span>A better way to develop the cli</span>
              <br />
              <i>inspired by <a target='blank' href='https://github.com/tj/commander.js/'>commander.js</a> & <a target='blank' href='https://rocket.rs/'>rocket.rs</a></i>
            </div>
          </div>
        </div>
        <div className='down shutter'></div>
      </div>

      <div className={upflowClass}>
        <ProgressiveCodeBlock code={code} />
      </div>

      <div className={insideClass}>
        {
          children
        }
      </div>
    </div>
  );
}
