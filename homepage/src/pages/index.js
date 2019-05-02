import React from 'react';
import { useState } from 'react';
import doc from "../../public/readme.html";
import "./styles/index.css";
import Shades from '../components/shades';

const code = `
#![feature(proc_macro_hygiene)]

use commander_rust::{option, command, entry, Cli, run};

#[option(-c, --cn, "Chinese")]
#[option(-e, --en, "English")]
#[option(-j, --jp, "Japanese")]
#[command(hello, "Say hello")]
fn hello(cli: Cli) {
    if cli.has("cn") {
        println!("你好，世界");
    } else if cli.has("en") {
        println!("hello, world!");
    } else if cli.has("jp") {
        println!("こんにちは、世界");
    }
}

#[entry]
fn main() { run!(); }
`;

export default function Index() {

  const [ state, setState ] = useState(false);

  return (
    <React.Fragment>
      {/* <Shutter expand={state} code={code}>
        <div dangerouslySetInnerHTML={{
          __html: doc,
        }} />
      </Shutter> */}
      <Shades expand={state} code={code} content={ doc } />

      <div className='next' onClick={() => setState(!state)}>
        <div className='btn'>+</div>
      </div>
    </React.Fragment>
  );
}
