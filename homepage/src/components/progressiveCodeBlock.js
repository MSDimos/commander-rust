import React from 'react';
import { useEffect, useState } from 'react';
import Highlight from 'react-highlight';
import 'highlight.js/styles/atom-one-dark.css';

import './styles/progressiveCodeBlock.css';

const raf = window.requestAnimationFrame;
const easing = (t, b, c, d) => {
if ((t /= d / 2) < 1) return -c / 2 * (Math.sqrt(1 - t * t) - 1) + b;
    return c / 2 * (Math.sqrt(1 - (t -= 2) * t) + 1) + b;
};
let start = 0;
const duration = 6000;

function step(time) {
  if (!start) {
    start = time;
  }

  let d = time - start;

  if (d < duration) {
    this.callback(parseInt(easing(d, 0, this.length, duration), 10));
    raf(step.bind(this));
  } else {
    this.callback(this.length);
  }
}

function run(callback, code) {
  let s = step.bind({
    length: code.length,
    callback,
  });
  raf(s);
}

export default function ProgressiveCodeBlock(props = {
  code: '',
  children: '',
}) {
  const { code } = props;

  const [ end, setEnd] = useState(0);
  useEffect(() => {
    run(setEnd, code);
  });

  return (
    <Highlight className='rust'>
      {
        code.slice(0, end)
      }
    </Highlight>
  );
}
