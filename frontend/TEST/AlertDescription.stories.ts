import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDescription from '../components/ui/alert/AlertDescription.vue';

const meta = {
  title: 'AlertDescription',
  component: AlertDescription,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDescription>;

export default meta;
type Story = StoryObj<typeof AlertDescription>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};