import { useState, useEffect, useCallback } from 'react';
const App = () => {
    const [name, setName] = useState('');
    const [another, setAnother] = useState(3);
    const [isOpen, setIsOpen] = useState();

    useEffect(() => {
    document.title = `You clicked ${count} times`;
  }, []);
    useEffect(() => {
    // alert("This component was mounted");
    document.title = `You clicked ${count} times`;
     
    if (isOpen && !prevProps.isOpen) {
      alert("You just opened the modal!");
    }
  }, [isOpen]);
    const alertNameHandler = useCallback(() => {
    alert(name);
  }, [name]);
    const handleNameInputHandler = useCallback(e => {
    setName(e.target.value);
    setAnother("cooler");
  }, []);
    const asyncAlertHandler = useCallback(async () => {
    await alert("async alert");
  }, []);

    return (
      <div>
        <h3>This is a Class Component</h3>
        <input
          type="text"
          onChange={handleNameInputHandler}
          value={name}
          placeholder="Your Name"
        />
        <button onClick={alertNameHandler}>
          Alert
        </button>
        <button onClick={asyncAlertHandler}>
          Alert
        </button>
      </div>
    ); 
};
App.foo = 1;
    App.fooBar = 21;
    App.bar = (input) => {
      console.log(input);
  };
    App.another = input => {
      console.error(input);
  };
