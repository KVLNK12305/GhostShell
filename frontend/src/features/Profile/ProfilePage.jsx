import React, {useState} from "react";
import ProfileHeader from './ProfileHeader';
import ProfileForm from './ProfileForm';

const ProfilePage = () => {
    const [user, setUser] = useState({
        name: "John Doe",
        email: "johnDoe@gmail.com",
        bio: "A short bio about John Doe."
    });

    const handleUpdateUser = (updatedUser) => {
        console.log("Updating user:", updatedUser);
        setUser(updatedUser);
        // TODO: Add logic to persist the updated user data, e.g., API call

    };
    
}