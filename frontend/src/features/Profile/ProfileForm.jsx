import React, { useState } from 'react';

const ProfileForm = ({ initialData, onSave }) => {
  const [formData, setFormData] = useState(initialData);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    onSave(formData);
  };

  return (
    <form onSubmit={handleSubmit} className="p-6 space-y-6">
      {/* Username Field */}
      <div className="flex flex-col space-y-2">
        <label className="text-xs uppercase text-green-700 tracking-wider">
          Codename_
        </label>
        <input
          type="text"
          name="username"
          value={formData.username}
          onChange={handleChange}
          className="cyber-input w-full p-2 text-lg"
          autoComplete="off"
        />
      </div>

      {/* Bio Field */}
      <div className="flex flex-col space-y-2">
        <label className="text-xs uppercase text-green-700 tracking-wider">
           Mission_Objective_
        </label>
        <textarea
          name="bio"
          value={formData.bio}
          onChange={handleChange}
          rows="3"
          className="cyber-input w-full p-2 text-md resize-none"
        />
      </div>

      {/* Submit Button */}
      <div className="pt-4 flex justify-end">
        <button
          type="submit"
          className="px-6 py-2 bg-green-900 hover:bg-green-700 text-green-100 font-mono border border-green-500 transition-all hover:shadow-[0_0_10px_#33ff00]"
        >
          [ UPDATE_RECORD ]
        </button>
      </div>
    </form>
  );
};

export default ProfileForm;