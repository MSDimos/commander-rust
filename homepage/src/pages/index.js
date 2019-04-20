import React from 'react';
import Shutter from '../components/shutter';
import { useState } from 'react';

import "./styles/index.css";

const code = `
#![feature(proc_macro_hygiene)]

use commander_rust::{command, option, entry, take, Cmd};

#[option(-s, --simple [dir], "simplify sth")]
#[option(-r, --recursive [dir...], "recursively")]
#[command(rmdir <dir> [otherDirs...], "remove files and directories")]
fn rmdir(dir: i32, other_dirs: Option<Vec<bool>>, cmd: Cmd) {
    println!("dir is {}, other_dirs is {:#?}", dir, other_dirs);
}


#[option(-s, --simple [dir], "simplify sth")]
#[option(-r, --recursive, "recursively")]
#[entry]
fn main() {
    take![rmdir, copy];
}
`;

export default function Index() {

  const [ state, setState ] = useState(false);

  return (
    <React.Fragment>
      <Shutter expand={state} code={code}>
        <p>Hello world!</p>
      </Shutter>
      <div className='next' onClick={() => setState(!state)}>
        <div className='btn'>+</div>
      </div>
    </React.Fragment>
  );
}
