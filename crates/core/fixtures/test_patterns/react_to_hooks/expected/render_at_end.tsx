import { useEffect } from "react";

const Link = (props) => {
    

    useEffect(() => { 
    return () => {
    console.log("unmounted");
  };
});

    const { href } = props;

    return <a href={href}>Link Text</a>; 
};



