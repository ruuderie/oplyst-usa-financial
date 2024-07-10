import type { Meta, StoryObj } from '@storybook/vue3';

import AvatarFallback from '../components/ui/avatar/AvatarFallback.vue';

const meta = {
  title: 'AvatarFallback',
  component: AvatarFallback,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AvatarFallback>;

export default meta;
type Story = StoryObj<typeof AvatarFallback>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};