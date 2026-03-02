import React from 'react';

const ProfileHeader = ({ username, role }) => {
  return (
    <div className="flex items-center space-x-6 p-6 border-b border-green-900">
      {/* Abstract Avatar */}
      <div className="w-20 h-20 bg-green-900 rounded-full flex items-center justify-center border-2 border-green-500 shadow-[0_0_15px_rgba(34,197,94,0.5)]">
        <span className="text-3xl font-bold text-black">
          {username ? username[0].toUpperCase() : '?'}
        </span>
      </div>

      <div>
        <h1 className="text-2xl font-bold tracking-widest uppercase">{username || 'Unknown User'}</h1>
        <div className="flex items-center space-x-2 mt-1">
          <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
          <p className="text-sm text-green-700 font-mono uppercase tracking-widest">
            {role || 'Clearance Level: 0'}
          </p>
        </div>
      </div>
    </div>
  );
};

export default ProfileHeader;