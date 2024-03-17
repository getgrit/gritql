import { useRef } from 'react';

import { observer } from "mobx-react";

const SampleComponent = observer(() => {
    

    const viewState = useRef(new ViewState());

    return (
        <p>This component has a <span onClick={viewState.click}>ViewState</span></p>
    ); 
});


