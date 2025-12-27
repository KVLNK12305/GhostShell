import React, { useState } from "react";
import { Link } from "react-router-dom";
import ProfileHeader from "./ProfileHeader";
import ProfileForm from "./ProfileForm";

const ProfilePage = () => {
  const [user, setUser] = useState({
    name: "John Doe",
    email: "johnDoe@gmail.com",
    bio: "A short bio about John Doe.",
  });

  const handleUpdateUser = (updatedUser) => {
    console.log("Updating user:", updatedUser);
    setUser({ ...updatedUser });
    // TODO: Persist updated user (API call)
  };

  return (
    <div className="min-h-screen bg-black text-green-500 font-mono p-8 flex items-center justify-center relative overflow-hidden">
      
      {/* Background Decor */}
      <div className="absolute top-0 left-0 w-full h-full pointer-events-none scanline opacity-20"></div>

      {/* Main Card */}
      <div className="cyber-card w-full max-w-2xl relative z-10 bg-black">
        
        {/* Navigation / Close */}
        <div className="absolute top-4 right-4">
          <Link to="/" className="text-xs text-green-700 hover:text-green-400">
            [ X ] CLOSE_DOSSIER
          </Link>
        </div>

        <ProfileHeader username={user.name} role={user.email} />

        <ProfileForm initialData={user} onSave={handleUpdateUser} />

        {/* Footer Decor */}
        <div className="bg-green-900/20 p-2 text-center text-xs text-green-800 border-t border-green-900">
          SYSTEM_ID: GHOST_SHELL_V1.0 // UNSECURE CONNECTION
        </div>
      </div>
    </div>
  );
};

export default ProfilePage;
