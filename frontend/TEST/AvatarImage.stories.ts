import type { Meta, StoryObj } from '@storybook/vue3';

import AvatarImage from '../components/ui/avatar/AvatarImage.vue';

const meta = {
  title: 'AvatarImage',
  component: AvatarImage,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AvatarImage>;

export default meta;
type Story = StoryObj<typeof AvatarImage>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};