import type { Meta, StoryObj } from '@storybook/vue3';

import Avatar from '../components/ui/avatar/Avatar.vue';

const meta = {
  title: 'Avatar',
  component: Avatar,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Avatar>;

export default meta;
type Story = StoryObj<typeof Avatar>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};